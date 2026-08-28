use crate::posts::Post;

pub fn generate_sitemap(posts: &[Post], site_url: &str) -> String {
  let mut sitemap = String::from(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
  );

  // Add homepage
  sitemap.push_str(&format!(
    r#"  <url>
    <loc>{site_url}/</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#
  ));

  // Add posts
  for post in posts {
    sitemap.push_str(&format!(
      r#"  <url>
    <loc>{site_url}/posts/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
"#,
      post.slug, post.metadata.date
    ));
  }

  // Add unique tags
  let mut tags: Vec<String> = posts
    .iter()
    .flat_map(|post| post.metadata.tags.clone())
    .collect();
  tags.sort();
  tags.dedup();

  for tag in tags {
    sitemap.push_str(&format!(
      r#"  <url>
    <loc>{site_url}/tags/{}</loc>
    <changefreq>weekly</changefreq>
    <priority>0.6</priority>
  </url>
"#,
      tag
    ));
  }

  sitemap.push_str("</urlset>");
  sitemap
}

pub fn generate_robots_txt(site_url: &str) -> String {
  format!("User-agent: *\nAllow: /\n\nSitemap: {site_url}/sitemap.xml\n")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sitemap_and_robots_use_the_configured_site_url() {
    let sitemap = generate_sitemap(&[], "https://example.com");
    let robots = generate_robots_txt("https://example.com");

    assert!(sitemap.contains("<loc>https://example.com/</loc>"));
    assert!(robots.contains("Sitemap: https://example.com/sitemap.xml"));
  }
}
