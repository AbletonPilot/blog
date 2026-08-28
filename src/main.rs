#[cfg(feature = "ssr")]
fn write_static_metadata(
  site_root: &std::path::Path,
  posts: &[blog::posts::Post],
  site_url: &str,
) -> std::io::Result<()> {
  std::fs::create_dir_all(site_root)?;
  std::fs::write(
    site_root.join("rss.xml"),
    blog::rss::generate_rss(posts, site_url),
  )?;
  std::fs::write(
    site_root.join("sitemap.xml"),
    blog::sitemap::generate_sitemap(posts, site_url),
  )?;
  std::fs::write(
    site_root.join("robots.txt"),
    blog::sitemap::generate_robots_txt(site_url),
  )?;
  Ok(())
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use axum::{
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
  };
  use blog::app::*;
  use blog::posts::load_posts;
  use blog::rss::generate_rss;
  use blog::sitemap::{generate_robots_txt, generate_sitemap};
  use leptos::logging::log;
  use leptos::prelude::*;
  use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};

  let conf = get_configuration(None).unwrap();
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  let (routes, static_routes) = generate_route_list_with_ssg({
    let leptos_options = leptos_options.clone();
    move || shell(leptos_options.clone())
  });
  static_routes.generate(&leptos_options).await;

  if std::env::var_os("LEPTOS_SSG_ONLY").is_some() {
    let posts = load_posts();
    write_static_metadata(
      std::path::Path::new(leptos_options.site_root.as_ref()),
      &posts,
      blog::SITE_URL,
    )
    .expect("failed to write static metadata");
    return;
  }

  // Cache control middleware for static assets
  async fn cache_middleware(req: axum::extract::Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let mut response = next.run(req).await;

    // Apply cache headers to static assets
    if path.starts_with("/pkg/")
      || path.ends_with(".css")
      || path.ends_with(".js")
      || path.ends_with(".wasm")
    {
      response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable"),
      );
    } else if path.ends_with(".xml") || path.ends_with(".txt") {
      response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600"),
      );
    }

    response
  }

  // RSS handler
  async fn rss_handler() -> Response {
    let posts = load_posts();
    let rss_content = generate_rss(&posts, blog::SITE_URL);
    (
      StatusCode::OK,
      [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
      rss_content,
    )
      .into_response()
  }

  // Sitemap handler
  async fn sitemap_handler() -> Response {
    let posts = load_posts();
    let sitemap_content = generate_sitemap(&posts, blog::SITE_URL);
    (
      StatusCode::OK,
      [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
      sitemap_content,
    )
      .into_response()
  }

  // Robots.txt handler
  async fn robots_handler() -> Response {
    let robots_content = generate_robots_txt(blog::SITE_URL);
    (
      StatusCode::OK,
      [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
      robots_content,
    )
      .into_response()
  }

  let app = Router::new()
    .route("/rss.xml", axum::routing::get(rss_handler))
    .route("/sitemap.xml", axum::routing::get(sitemap_handler))
    .route("/robots.txt", axum::routing::get(robots_handler))
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .layer(middleware::from_fn(cache_middleware))
    .with_state(leptos_options);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
  // no client-side main function
  // unless we want this to work with e.g., Trunk for pure client-side testing
  // see lib.rs for hydration function instead
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
  use super::*;
  use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
  };

  #[test]
  fn writes_static_metadata_files() {
    let nonce = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let root = std::env::temp_dir().join(format!("blog-static-{nonce}"));

    write_static_metadata(&root, &[], "https://example.com").unwrap();

    assert!(fs::read_to_string(root.join("rss.xml"))
      .unwrap()
      .contains("https://example.com/rss.xml"));
    assert!(fs::read_to_string(root.join("sitemap.xml"))
      .unwrap()
      .contains("https://example.com/"));
    assert!(fs::read_to_string(root.join("robots.txt"))
      .unwrap()
      .contains("https://example.com/sitemap.xml"));

    fs::remove_dir_all(root).unwrap();
  }
}
