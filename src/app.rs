use crate::components::{AboutPage, ArchivePage, Giscus, PostSummaryCard};
use crate::posts::{Post, PostSummary};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  path, StaticSegment,
};

// Global search context
#[derive(Clone, Copy)]
pub struct SearchContext {
  pub query: RwSignal<String>,
  pub current_page: RwSignal<usize>,
}

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
  let search_ctx = expect_context::<SearchContext>();
  let navigate = leptos_router::hooks::use_navigate();

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
              on:input=move |ev| {
                search_ctx.query.set(event_target_value(&ev));
                search_ctx.current_page.set(1);
                // Navigate to home page when searching from other pages
                navigate("/", Default::default());
              }
              prop:value=move || search_ctx.query.get()
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
      <div>
        <p>"© 2024 Junmo. All rights reserved."</p>
      </div>
        <p>
          "Built with Rust and Leptos | "
          <span class="footer-links">
            <a href="https://github.com/AbletonPilot" target="_blank">
              "GitHub"
            </a>
            <span>" | "</span>
            <a href="https://ko-fi.com/abletonpilot" target="_blank">
              "Ko-fi"
            </a>
          </span>
        </p>
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
        <link rel="alternate" type="application/rss+xml" title="Junmo's Blog RSS Feed" href="/rss.xml"/>
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

  // Provide global search context
  let search_ctx = SearchContext {
    query: RwSignal::new(String::new()),
    current_page: RwSignal::new(1),
  };
  provide_context(search_ctx);

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
  let search_ctx = expect_context::<SearchContext>();
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
        <div class="header-intro">
          <span class="wave">"👋"</span>
          <span class="greeting">" Welcome to Junmo's Blog"</span>
        </div>
        <h1>"Junmo's Blog"</h1>
        <p class="tagline">"Thoughts on programming and technology"</p>
        <div class="social-icons">
          <a href="https://github.com/AbletonPilot" target="_blank" rel="noopener noreferrer" aria-label="GitHub" class="social-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
          </a>
          <a href="https://www.linkedin.com/in/junmo-son-46093a1b9/" target="_blank" rel="noopener noreferrer" aria-label="LinkedIn" class="social-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
            </svg>
          </a>
          <a href="https://ko-fi.com/abletonpilot" target="_blank" rel="noopener noreferrer" aria-label="Ko-fi" class="social-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="none">
              <path d="M20.216 6.415l-.132-.666c-.119-.598-.388-1.163-1.001-1.379-.197-.069-.42-.098-.57-.241-.152-.143-.196-.366-.231-.572-.065-.378-.125-.756-.192-1.133-.057-.325-.102-.69-.25-.987-.195-.4-.597-.634-.996-.788a5.723 5.723 0 00-.626-.194c-1-.263-2.05-.36-3.077-.416a25.834 25.834 0 00-3.7.062c-.915.083-1.88.184-2.75.5-.318.116-.646.256-.888.501-.297.302-.393.77-.177 1.146.154.267.415.456.692.58.36.162.737.284 1.123.366 1.075.238 2.189.331 3.287.37 1.218.05 2.437.01 3.65-.118.299-.033.598-.073.896-.119.352-.054.578-.513.474-.834-.124-.383-.457-.531-.834-.473-.466.074-.96.108-1.382.146-1.177.08-2.358.082-3.536.006a22.228 22.228 0 01-1.157-.107c-.086-.01-.18-.025-.258-.036-.243-.036-.484-.08-.724-.13-.111-.027-.111-.185 0-.212h.005c.277-.06.557-.108.838-.147h.002c.131-.009.263-.032.394-.048a25.076 25.076 0 013.426-.12c.674.019 1.347.067 2.017.144l.228.031c.267.04.533.088.798.145.392.085.895.113 1.07.542.055.137.08.288.111.431l.319 1.484a.237.237 0 01-.199.284h-.003c-.037.006-.075.01-.112.015a36.704 36.704 0 01-4.743.295 37.059 37.059 0 01-4.699-.304c-.14-.017-.293-.042-.417-.06-.326-.048-.649-.108-.973-.161-.393-.065-.768-.032-1.123.161-.29.16-.527.404-.675.701-.154.316-.199.66-.267 1-.069.34-.176.707-.135 1.056.087.753.613 1.365 1.37 1.502a39.69 39.69 0 0011.343.376.483.483 0 01.535.53l-.071.697-1.018 9.907c-.041.41-.047.832-.125 1.237-.122.637-.553 1.028-1.182 1.171-.577.131-1.165.2-1.756.205-.656.004-1.31-.025-1.966-.022-.699.004-1.556-.06-2.095-.58-.475-.458-.54-1.174-.605-1.793l-.731-7.013-.322-3.094c-.037-.351-.286-.695-.678-.678-.336.015-.718.3-.678.679l.228 2.185.949 9.112c.147 1.344 1.174 2.068 2.446 2.272.742.12 1.503.144 2.257.156.966.016 1.942.053 2.892-.122 1.408-.258 2.465-1.198 2.616-2.657.34-3.332.683-6.663 1.024-9.995l.215-2.087a.484.484 0 01.39-.426c.402-.078.787-.212 1.074-.518.455-.488.546-1.124.385-1.766zm-1.478.772c-.145.137-.363.201-.578.233-2.416.359-4.866.54-7.308.46-1.748-.06-3.477-.254-5.207-.498-.17-.024-.353-.055-.47-.18-.22-.236-.111-.71-.054-.995.052-.26.152-.609.463-.646.484-.057 1.046.148 1.526.22.577.088 1.156.159 1.737.212 2.48.226 5.002.19 7.472-.14.45-.06.899-.13 1.345-.21.399-.072.84-.206 1.08.206.166.281.188.657.162.974a.544.544 0 01-.169.364z"/>
            </svg>
          </a>
        </div>
      </header>

      <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
        {move || {
          posts.get().map(|posts| {
            let query = search_ctx.query.get().to_lowercase();
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
              let current = search_ctx.current_page.get();
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
                        disabled=move || search_ctx.current_page.get() == 1
                        on:click=move |_| search_ctx.current_page.update(|p| *p = (*p - 1).max(1))
                      >
                        "Previous"
                      </button>
                      <span class="pagination-info">
                        {format!("Page {} of {}", current, total_pages)}
                      </span>
                      <button
                        class="pagination-btn"
                        disabled=move || search_ctx.current_page.get() >= total_pages
                        on:click=move |_| search_ctx.current_page.update(|p| *p = (*p + 1).min(total_pages))
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
