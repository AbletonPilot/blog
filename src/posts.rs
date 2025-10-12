use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostMetadata {
  pub title: String,
  pub date: String,
  pub tags: Vec<String>,
  pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
  pub slug: String,
  pub metadata: PostMetadata,
  pub content: String,
}

impl Post {
  pub fn from_markdown(slug: String, markdown_content: &str) -> Result<Self, String> {
    let parts: Vec<&str> = markdown_content.split("---").collect();

    if parts.len() < 3 {
      return Err("Invalid markdown format: missing front matter".to_string());
    }

    let front_matter = parts[1].trim();
    let content = parts[2..].join("---").trim().to_string();

    let metadata: PostMetadata = serde_yaml::from_str(front_matter)
      .map_err(|e| format!("Failed to parse front matter: {}", e))?;

    let html_content = markdown_to_html(&content);

    Ok(Post {
      slug,
      metadata,
      content: html_content,
    })
  }
}

fn markdown_to_html(markdown: &str) -> String {
  use pulldown_cmark::{html, Options, Parser};

  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_TASKLISTS);

  #[cfg(feature = "ssr")]
  {
    use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};
    use syntect::highlighting::ThemeSet;
    use syntect::html::highlighted_html_for_string;
    use syntect::parsing::SyntaxSet;

    let parser = Parser::new_ext(markdown, options);
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    let events: Vec<Event> = parser
      .filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
          in_code_block = true;
          code_block_lang = lang.to_string();
          code_block_content.clear();
          None
        }
        Event::End(TagEnd::CodeBlock) if in_code_block => {
          in_code_block = false;
          let syntax = ss
            .find_syntax_by_token(&code_block_lang)
            .unwrap_or_else(|| ss.find_syntax_plain_text());
          let html = highlighted_html_for_string(&code_block_content, &ss, syntax, theme)
            .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code_block_content));
          Some(Event::Html(html.into()))
        }
        Event::Text(text) if in_code_block => {
          code_block_content.push_str(&text);
          None
        }
        _ => Some(event),
      })
      .collect();

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
  }

  #[cfg(not(feature = "ssr"))]
  {
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
  }
}

#[cfg(feature = "ssr")]
pub fn load_posts() -> Vec<Post> {
  use std::fs;
  use std::path::Path;

  let posts_dir = Path::new("posts");

  if !posts_dir.exists() {
    eprintln!("Posts directory does not exist");
    return vec![];
  }

  let mut posts = Vec::new();

  if let Ok(entries) = fs::read_dir(posts_dir) {
    for entry in entries.flatten() {
      let path = entry.path();

      if path.extension().and_then(|s| s.to_str()) == Some("md") {
        if let Ok(content) = fs::read_to_string(&path) {
          let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

          match Post::from_markdown(filename, &content) {
            Ok(post) => posts.push(post),
            Err(e) => eprintln!("Error parsing post {}: {}", path.display(), e),
          }
        }
      }
    }
  }

  posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
  posts
}

#[cfg(not(feature = "ssr"))]
pub fn load_posts() -> Vec<Post> {
  vec![]
}
