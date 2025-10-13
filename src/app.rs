use crate::components::{AboutPage, ArchivePage, Giscus, PostSummaryCard};
use crate::posts::{Post, PostSummary};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  path, StaticSegment,
};

#[server]
pub async fn get_post_summaries() -> Result<Vec<PostSummary>, ServerFnError> {
  Ok(crate::posts::load_post_summaries())
}

#[server]
pub async fn get_posts_by_tag_summaries(tag: String) -> Result<Vec<PostSummary>, ServerFnError> {
  let posts = crate::posts::load_post_summaries();
  Ok(
    posts
      .into_iter()
      .filter(|p| p.metadata.tags.iter().any(|t| t == &tag))
      .collect(),
  )
}

#[server]
pub async fn get_rss() -> Result<String, ServerFnError> {
  let posts = crate::posts::load_posts();
  Ok(crate::rss::generate_rss(&posts))
}

#[server]
pub async fn get_sitemap() -> Result<String, ServerFnError> {
  let posts = crate::posts::load_posts();
  Ok(crate::sitemap::generate_sitemap(&posts))
}

#[server]
pub async fn get_robots_txt() -> Result<String, ServerFnError> {
  Ok(crate::sitemap::generate_robots_txt())
}

#[server]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
  Ok(crate::posts::load_posts())
}

#[server]
pub async fn get_post_by_slug(slug: String) -> Result<Option<Post>, ServerFnError> {
  let posts = crate::posts::load_posts();
  Ok(posts.into_iter().find(|p| p.slug == slug))
}

#[server]
pub async fn get_posts_by_tag(tag: String) -> Result<Vec<Post>, ServerFnError> {
  let posts = crate::posts::load_posts();
  Ok(
    posts
      .into_iter()
      .filter(|p| p.metadata.tags.iter().any(|t| t == &tag))
      .collect(),
  )
}

#[component]
fn SiteHeader() -> impl IntoView {
  let is_dark = RwSignal::new(true);

  let toggle_theme = move |_| {
    is_dark.update(|dark| *dark = !*dark);

    #[cfg(target_arch = "wasm32")]
    {
      let win = window();
      if let Some(document) = win.document() {
        if let Some(body) = document.body() {
          if is_dark.get() {
            let _ = body.class_list().remove_1("light-mode");
          } else {
            let _ = body.class_list().add_1("light-mode");
          }
        }
      }
    }
  };
  let (menu_open, set_menu_open) = signal(false);

  view! {
    <header class="site-header">
      <nav class="container">
        <div class="nav-brand">
          <a href="/">"Junmo's Blog"</a>
        </div>

        // Desktop navigation
        <div class="nav-left desktop-nav">
          <ul class="nav-links">
            <li><a href="/archive">"Archive"</a></li>
            <li><a href="/about">"About"</a></li>
          </ul>
        </div>

        <div class="nav-right">
          <div class="search-container">
            <input
              type="text"
              placeholder="Search..."
              class="search-input"
            />
            <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <path d="m21 21-4.35-4.35"></path>
            </svg>
          </div>

          <button class="theme-toggle" on:click=toggle_theme>
            {move || if is_dark.get() {
              view! {
                <svg
                  id="moon"
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                </svg>
              }.into_any() } else {
                view! {
                  <svg
                    id="sun"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <circle cx="12" cy="12" r="5"></circle>
                    <line x1="12" y1="1" x2="12" y2="3"></line>
                    <line x1="12" y1="21" x2="12" y2="23"></line>
                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                    <line x1="1" y1="12" x2="3" y2="12"></line>
                    <line x1="21" y1="12" x2="23" y2="12"></line>
                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                  </svg>
                }.into_any()
              }
            }
          </button>

          <button
            class="hamburger-btn mobile-nav"
            on:click=move |_| set_menu_open.update(|open| *open = !*open)
          >
            <span class="hamburger-line"></span>
            <span class="hamburger-line"></span>
            <span class="hamburger-line"></span>
          </button>
        </div>

        // Mobile menu
        <div class=move || format!("mobile-menu {}", if menu_open.get() { "open" } else { "" })>
          <ul class="mobile-nav-links">
            <li><a href="/archive" on:click=move |_| set_menu_open.set(false)>"Archive"</a></li>
            <li><a href="/about" on:click=move |_| set_menu_open.set(false)>"About"</a></li>
          </ul>
        </div>
      </nav>
    </header>
  }
}

#[component]
fn SiteFooter() -> impl IntoView {
  view! {
    <footer class="site-footer">
      <div class="container">
        <p>"Built with Rust and Leptos"</p>
        <div class="footer-links">
          <a href="https://github.com/AbletonPilot" target="_blank">"GitHub"</a>
          <span>" | "</span>
          <a href="/rss.xml">"RSS"</a>
        </div>
      </div>
    </footer>
  }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        <link rel="dns-prefetch" href="https://blog.abletonpilot.me"/>
        <AutoReload options=options.clone() />
        <HydrationScripts options/>
        <MetaTags/>
      </head>
      <body>
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/blog.css"/>
    // sets the document title
    <Title text="Junmo's Blog"/>
    // content for this welcome page
    <Router>
      <SiteHeader/>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
          <Route path=StaticSegment("archive") view=ArchivePage/>
          <Route path=StaticSegment("about") view=AboutPage/>
          <Route path=path!("/posts/:slug") view=PostPage/>
          <Route path=path!("/tags/:tag") view=TagPage/>
          <Route path=StaticSegment("sitemap.xml") view=SitemapPage/>
          <Route path=StaticSegment("robots.txt") view=RobotsPage/>
          <Route path=StaticSegment("rss.xml") view=RssPage/>
        </Routes>
      </main>
      <SiteFooter/>
    </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  let posts = Resource::new(
    || (),
    |_| async move { get_post_summaries().await.unwrap_or_default() },
  );
  let search_query = RwSignal::new(String::new());
  let current_page = RwSignal::new(1);
  let posts_per_page = 10;

  view! {
    <Title text="Junmo's Blog - Thoughts on programming and technology"/>
    <Meta name="description" content="A blog about programming, technology, and software development. Sharing insights and experiences in web development, Rust, and more."/>
    <Meta name="keywords" content="programming, technology, software development, rust, web development, leptos"/>
    <Meta property="og:type" content="website"/>
    <Meta property="og:title" content="Junmo's Blog"/>
    <Meta property="og:description" content="A blog about programming, technology, and software development"/>
    <Meta property="og:url" content="https://blog.abletonpilot.me/"/>
    <Meta property="og:site_name" content="Junmo's Blog"/>
    <Meta name="twitter:card" content="summary"/>
    <Meta name="twitter:title" content="Junmo's Blog"/>
    <Meta name="twitter:description" content="A blog about programming, technology, and software development"/>
    <link rel="canonical" href="https://blog.abletonpilot.me/"/>


    <div class="container">
      <header class="blog-header">
        <h1>"Junmo's Blog"</h1>
        <p>"Thoughts on programming and technology"</p>
      </header>

      <div class="search">
        <input
          type="text"
          placeholder="Search posts..."
          on:input=move |ev| {
            search_query.set(event_target_value(&ev));
            current_page.set(1);
          }
          prop:value=move || search_query.get()
        />
      </div>

      <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
        {move || {
          posts.get().map(|posts| {
            let query = search_query.get().to_lowercase();
            let filtered_posts: Vec<PostSummary> = if query.is_empty() {
              posts
            } else {
              posts.into_iter().filter(|post| {
                post.metadata.title.to_lowercase().contains(&query) ||
                post.metadata.description.to_lowercase().contains(&query) ||
                post.metadata.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
              }).collect()
            };

            if filtered_posts.is_empty() {
              view! {
                <div class="no-posts">
                  <p>"No posts found matching your search."</p>
                </div>
              }.into_any()
            } else {
              let total_posts = filtered_posts.len();
              let total_pages = (total_posts + posts_per_page - 1) / posts_per_page;
              let current = current_page.get();
              let start_idx = (current - 1) * posts_per_page;
              let paginated_posts: Vec<PostSummary> = filtered_posts.into_iter().skip(start_idx).take(posts_per_page).collect();

              view! {
                <div class="posts-list">
                  {paginated_posts.iter().map(|post| view! { <PostSummaryCard post=post.clone() /> }).collect_view()}
                </div>
                {if total_pages > 1 {
                  view! {
                    <div class="pagination">
                      <button
                        class="pagination-btn"
                        disabled=move || current_page.get() == 1
                        on:click=move |_| current_page.update(|p| *p = (*p - 1).max(1))
                      >
                        "Previous"
                      </button>
                      <span class="pagination-info">
                        {format!("Page {} of {}", current, total_pages)}
                      </span>
                      <button
                        class="pagination-btn"
                        disabled=move || current_page.get() >= total_pages
                        on:click=move |_| current_page.update(|p| *p = (*p + 1).min(total_pages))
                      >
                        "Next"
                      </button>
                    </div>
                  }.into_any()
                } else {
                  view! { <div></div> }.into_any()
                }}
              }.into_any()
            }
          })
        }}
      </Suspense>
    </div>
  }
}

#[component]
fn PostPage() -> impl IntoView {
  let params = leptos_router::hooks::use_params_map();
  let slug = move || params.read().get("slug").unwrap_or_default();

  let post = Resource::new(
    move || slug(),
    |slug| async move { get_post_by_slug(slug).await.ok().flatten() },
  );

  view! {
    <div class="container">
      <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
        {move || {
          post.get().map(|post_opt| {
            match post_opt {
              Some(post) => {
                let title = post.metadata.title.clone();
                let date = post.metadata.date.clone();
                let tags = post.metadata.tags.clone();
                let content = post.content.clone();
                let description = post.metadata.description.clone();
                let page_title = format!("{} - Junmo's Blog", title);
                let og_url = format!("https://blog.abletonpilot.me/posts/{}", post.slug);

                view! {
                  <Title text=page_title.clone()/>
                  <Meta name="description" content=description.clone()/>
                  <Meta name="keywords" content=tags.join(", ")/>
                  <Meta property="og:type" content="article"/>
                  <Meta property="og:title" content=title.clone()/>
                  <Meta property="og:description" content=description.clone()/>
                  <Meta property="og:url" content=og_url.clone()/>
                  <Meta property="og:site_name" content="Junmo's Blog"/>
                  <Meta property="article:published_time" content=date.clone()/>
                  <Meta property="article:author" content="Junmo"/>
                  <Meta property="article:tag" content=tags.join(", ")/>
                  <Meta name="twitter:card" content="summary"/>
                  <Meta name="twitter:title" content=title.clone()/>
                  <Meta name="twitter:description" content=description/>
                  <Meta name="twitter:url" content=og_url/>

                  <article class="post-detail">
                    <header>
                      <h1>{title}</h1>
                      <div class="post-meta">
                        <span class="date">{date}</span>
                        <span class="tags">
                          {tags.iter().map(|tag| {
                            let tag_text = tag.clone();
                            let tag_link = tag.clone();
                            view! {
                              <a href=format!("/tags/{}", tag_link) class="tag">{tag_text}</a>
                            }
                          }).collect_view()}
                        </span>
                      </div>
                    </header>
                    <div class="post-content" inner_html=content></div>
                    <a href="/" class="back-link">"← Back to posts"</a>

                    // Comments section
                    <div class="comments-section">
                      <h2>"Comments"</h2>
                      <Giscus/>
                    </div>
                  </article>
                }.into_any()
              },
              None => view! {
                <Title text="Post Not Found - Junmo's Blog"/>
                <Meta name="description" content="The requested blog post could not be found"/>

                <div class="not-found">
                  <h1>"Post Not Found"</h1>
                  <p>"The post you are looking for does not exist."</p>
                  <a href="/">"← Back to posts"</a>
                </div>
              }.into_any(),
            }
          })
        }}
      </Suspense>
    </div>
  }
}

#[component]
fn TagPage() -> impl IntoView {
  let params = leptos_router::hooks::use_params_map();
  let tag = move || params.read().get("tag").unwrap_or_default();

  let posts = Resource::new(
    move || tag(),
    |tag| async move { get_posts_by_tag_summaries(tag).await.unwrap_or_default() },
  );

  view! {
    {move || {
      let current_tag = tag();
      let page_title = format!("Posts tagged with '{}' - Junmo's Blog", current_tag);
      let description = format!("All blog posts tagged with '{}' on Junmo's Blog", current_tag);

      view! {
        <Title text=page_title/>
        <Meta name="description" content=description/>
        <Meta name="keywords" content=format!("{}, programming, technology", current_tag)/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content=format!("Posts tagged with '{}'", current_tag)/>
        <Meta property="og:description" content=format!("All blog posts tagged with '{}' on Junmo's Blog", current_tag)/>
        <Meta property="og:site_name" content="Junmo's Blog"/>
        <Meta name="twitter:card" content="summary"/>
        <Meta name="twitter:title" content=format!("Posts tagged with '{}'", current_tag)/>
        <Meta name="twitter:description" content=format!("All blog posts tagged with '{}' on Junmo's Blog", current_tag)/>
      }
    }}

    <div class="container">
      <header class="tag-header">
        <h1>"Posts tagged with: " {move || tag()}</h1>
        <a href="/" class="back-link">"← All posts"</a>
      </header>

      <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
        {move || {
          posts.get().map(|posts| {
            if posts.is_empty() {
              view! {
                <div class="no-posts">
                  <p>"No posts found with this tag."</p>
                </div>
              }.into_any()
            } else {
              view! {
                <div class="posts-list">
                  {posts.iter().map(|post| view! { <PostSummaryCard post=post.clone() /> }).collect_view()}
                </div>
              }.into_any()
            }
          })
        }}
      </Suspense>
    </div>
  }
}

#[component]
fn SitemapPage() -> impl IntoView {
  let sitemap = Resource::new(
    || (),
    |_| async move { get_sitemap().await.unwrap_or_default() },
  );

  view! {
    <Suspense fallback=move || view! { <p>"Generating sitemap..."</p> }>
      {move || {
        sitemap.get().map(|content| {
          view! {
            <div style="white-space: pre-wrap; font-family: monospace; font-size: 12px;">
              {content}
            </div>
          }
        })
      }}
    </Suspense>
  }
}

#[component]
fn RobotsPage() -> impl IntoView {
  let robots = Resource::new(
    || (),
    |_| async move { get_robots_txt().await.unwrap_or_default() },
  );

  view! {
    <Suspense fallback=move || view! { <p>"Generating robots.txt..."</p> }>
      {move || {
        robots.get().map(|content| {
          view! {
            <div style="white-space: pre-wrap; font-family: monospace; font-size: 12px;">
              {content}
            </div>
          }
        })
      }}
    </Suspense>
  }
}

#[component]
fn RssPage() -> impl IntoView {
  let rss = Resource::new(
    || (),
    |_| async move { get_rss().await.unwrap_or_default() },
  );

  view! {
    <Suspense fallback=move || view! { <p>"Generating RSS feed..."</p> }>
      {move || {
        rss.get().map(|content| {
          view! {
            <div style="white-space: pre-wrap; font-family: monospace; font-size: 12px;">
              {content}
            </div>
          }
        })
      }}
    </Suspense>
  }
}
