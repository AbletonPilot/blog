# Cloudflare Pages SSG Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prerender every Rust/Leptos blog route in CI and deploy the resulting `target/site` directory to a pre-existing Cloudflare Pages Direct Upload project.

**Architecture:** Leptos static routing renders fixed routes plus every Markdown-derived slug and tag before Axum starts. `LEPTOS_SSG_ONLY=1` writes RSS, sitemap, and robots files and exits without listening; normal local development still runs Axum. GitHub Actions builds with the final `SITE_URL`, verifies the static output, and uploads only prebuilt assets with Wrangler.

**Tech Stack:** Rust 1.90.0, Leptos 0.8.x, Axum 0.8.x, cargo-leptos 0.2.47, GitHub Actions, Cloudflare Pages, Wrangler Action v4.

## Global Constraints

- Keep Rust, Leptos, the existing Markdown format, WASM hydration, search, theme controls, and Giscus.
- Add no Rust dependency; use existing crates and the Rust standard library.
- Keep `site-root = "target/site"` and the existing Axum path for local development.
- Build production assets with Rust 1.90.0 and cargo-leptos 0.2.47.
- Use a pre-existing Cloudflare Pages Direct Upload project; do not create, delete, deploy, or reconfigure external resources from local implementation work.
- Keep `.github/workflows/keepalive.yml` until the user has verified the real Pages origin.
- Treat `SITE_URL` and `CLOUDFLARE_PAGES_PROJECT` as public GitHub Actions variables and `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` as secrets.
- `SITE_URL` includes the scheme and has no trailing slash.

---

## File Map

- `src/lib.rs`: one compile-time canonical origin shared by rendered UI and static generators.
- `src/rss.rs`: RSS generation parameterized by origin.
- `src/sitemap.rs`: sitemap and robots generation parameterized by origin.
- `src/posts.rs`: pure slug and sorted-tag extraction for static route parameters.
- `src/app.rs`: static route declarations, canonical metadata, and full-document internal links.
- `src/main.rs`: route generation, static metadata writes, and SSG-only exit.
- `src/components/post_card.rs`: post/tag links that must bypass client routing.
- `src/components/archive_page.rs`: archive metadata and internal links.
- `src/components/about_page.rs`: about metadata and home link.
- `.github/workflows/deploy-pages.yml`: tested build, output checks, and Direct Upload.
- `docs/superpowers/specs/2026-08-28-cloudflare-pages-ssg-design.md`: correct locked-version output paths.

### Task 1: Parameterize the canonical site origin

**Files:**
- Modify: `src/lib.rs:1-14`
- Modify: `src/rss.rs:1-55`
- Modify: `src/sitemap.rs:1-65`
- Test: unit tests inside `src/rss.rs` and `src/sitemap.rs`

**Interfaces:**
- Produces: `pub const SITE_URL: &str`
- Produces: `pub fn generate_rss(posts: &[Post], site_url: &str) -> String`
- Produces: `pub fn generate_sitemap(posts: &[Post], site_url: &str) -> String`
- Produces: `pub fn generate_robots_txt(site_url: &str) -> String`

- [ ] **Step 1: Write failing generator tests**

Append to `src/rss.rs`:

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rss_uses_the_configured_site_url() {
    let rss = generate_rss(&[], "https://example.com");

    assert!(rss.contains("<link>https://example.com</link>"));
    assert!(rss.contains("href=\"https://example.com/rss.xml\""));
  }
}
```

Append to `src/sitemap.rs`:

```rust
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
```

- [ ] **Step 2: Run the tests and verify RED**

Run:

```bash
cargo test --features ssr configured_site_url
```

Expected: compilation fails because the three generators currently do not accept a `site_url` argument.

- [ ] **Step 3: Add the canonical constant and minimal generator parameters**

Add to `src/lib.rs`:

```rust
pub const SITE_URL: &str = match option_env!("SITE_URL") {
  Some(value) => value,
  None => "https://abletonpilot.onrender.com",
};
```

Change the generator signatures and interpolate the supplied origin:

```rust
pub fn generate_rss(posts: &[Post], site_url: &str) -> String {
  let mut rss = format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>AbletonPilot</title>
    <link>{site_url}</link>
    <description>A blog about programming, technology, and software development</description>
    <language>en-us</language>
    <atom:link href="{site_url}/rss.xml" rel="self" type="application/rss+xml"/>
    <lastBuildDate>Sun, 13 Oct 2025 00:00:00 GMT</lastBuildDate>
    <generator>Leptos RSS Generator</generator>
"#,
  );

  for post in posts.iter().take(20) {
    let pub_date = format_rfc2822_date(&post.metadata.date);
    let post_url = format!("{site_url}/posts/{}", post.slug);
    rss.push_str(&format!(
      r#"    <item>
      <title><![CDATA[{}]]></title>
      <link>{}</link>
      <guid>{}</guid>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
      <category><![CDATA[{}]]></category>
    </item>
"#,
      post.metadata.title,
      post_url,
      post_url,
      pub_date,
      post.metadata.description,
      post.metadata.tags.join(", ")
    ));
  }

  rss.push_str("  </channel>\n</rss>");
  rss
}
```

```rust
pub fn generate_sitemap(posts: &[Post], site_url: &str) -> String {
  let mut sitemap = String::from(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
  );
  sitemap.push_str(&format!(
    "  <url>\n    <loc>{site_url}/</loc>\n    <changefreq>daily</changefreq>\n    <priority>1.0</priority>\n  </url>\n"
  ));
  for post in posts {
    sitemap.push_str(&format!(
      "  <url>\n    <loc>{site_url}/posts/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
      post.slug, post.metadata.date
    ));
  }
  let mut tags: Vec<String> = posts
    .iter()
    .flat_map(|post| post.metadata.tags.clone())
    .collect();
  tags.sort();
  tags.dedup();
  for tag in tags {
    sitemap.push_str(&format!(
      "  <url>\n    <loc>{site_url}/tags/{tag}</loc>\n    <changefreq>weekly</changefreq>\n    <priority>0.6</priority>\n  </url>\n"
    ));
  }
  sitemap.push_str("</urlset>");
  sitemap
}

pub fn generate_robots_txt(site_url: &str) -> String {
  format!("User-agent: *\nAllow: /\n\nSitemap: {site_url}/sitemap.xml\n")
}
```

- [ ] **Step 4: Run the focused and full Rust tests**

Run:

```bash
cargo test --features ssr configured_site_url
cargo test --features ssr
```

Expected: both focused tests pass; the full Rust suite has zero failures.

- [ ] **Step 5: Commit Task 1**

```bash
git add src/lib.rs src/rss.rs src/sitemap.rs
git commit -m "refactor(site): parameterize canonical URL"
```

### Task 2: Derive deterministic static route parameters

**Files:**
- Modify: `src/posts.rs:1-345`
- Test: unit tests inside `src/posts.rs`

**Interfaces:**
- Consumes: existing `PostSummary`
- Produces: `pub fn post_slugs(posts: &[PostSummary]) -> Vec<String>`
- Produces: `pub fn post_tags(posts: &[PostSummary]) -> Vec<String>`

- [ ] **Step 1: Write failing slug and tag tests**

Append to `src/posts.rs`:

```rust
#[cfg(test)]
mod tests {
  use super::*;

  fn summary(slug: &str, tags: &[&str]) -> PostSummary {
    PostSummary {
      slug: slug.to_string(),
      metadata: PostMetadata {
        title: slug.to_string(),
        date: "2026-08-28T00:00:00".to_string(),
        display_date: "2026-08-28".to_string(),
        display_datetime: "2026-08-28 00:00".to_string(),
        tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
        description: String::new(),
      },
      thumbnail: None,
    }
  }

  #[test]
  fn static_post_slugs_preserve_post_order() {
    let posts = vec![summary("newer", &["rust"]), summary("older", &["linux"])];

    assert_eq!(post_slugs(&posts), vec!["newer", "older"]);
  }

  #[test]
  fn static_post_tags_are_sorted_and_deduplicated() {
    let posts = vec![
      summary("one", &["rust", "linux"]),
      summary("two", &["linux", "ai"]),
    ];

    assert_eq!(post_tags(&posts), vec!["ai", "linux", "rust"]);
  }
}
```

- [ ] **Step 2: Run the tests and verify RED**

Run:

```bash
cargo test --features ssr static_post_
```

Expected: compilation fails because `post_slugs` and `post_tags` do not exist.

- [ ] **Step 3: Implement the two pure helpers**

Add beside the `PostSummary` type in `src/posts.rs`:

```rust
pub fn post_slugs(posts: &[PostSummary]) -> Vec<String> {
  posts.iter().map(|post| post.slug.clone()).collect()
}

pub fn post_tags(posts: &[PostSummary]) -> Vec<String> {
  let mut tags: Vec<_> = posts
    .iter()
    .flat_map(|post| post.metadata.tags.iter().cloned())
    .collect();
  tags.sort();
  tags.dedup();
  tags
}
```

- [ ] **Step 4: Run focused and full Rust tests**

```bash
cargo test --features ssr static_post_
cargo test --features ssr
```

Expected: slug and tag tests pass; the full suite has zero failures.

- [ ] **Step 5: Commit Task 2**

```bash
git add src/posts.rs
git commit -m "feat(ssg): derive static route parameters"
```

### Task 3: Mark every public route for static generation

**Files:**
- Modify: `src/app.rs:1-710`
- Test: unit test inside `src/app.rs`

**Interfaces:**
- Consumes: `post_slugs`, `post_tags`, `get_post_summaries`
- Produces: five routes whose `SsrMode` is `Static`
- Produces: `StaticParamsMap` entries named exactly `slug` and `tag`

- [ ] **Step 1: Write a failing route-mode test**

Append to `src/app.rs`:

```rust
#[cfg(all(test, feature = "ssr"))]
mod tests {
  use super::*;
  use leptos::config::get_configuration;
  use leptos_axum::generate_route_list_with_ssg;
  use leptos_router::SsrMode;

  #[test]
  fn all_public_routes_are_static() {
    let options = get_configuration(None).unwrap().leptos_options;
    let (routes, _) = generate_route_list_with_ssg({
      let options = options.clone();
      move || shell(options.clone())
    });

    assert_eq!(routes.len(), 5);
    assert!(routes
      .iter()
      .all(|route| matches!(route.mode(), SsrMode::Static(_))));
  }
}
```

- [ ] **Step 2: Run the test and verify RED**

Run:

```bash
cargo test --features ssr app::tests::all_public_routes_are_static -- --exact
```

Expected: the assertion fails because current routes use the default non-static SSR mode.

- [ ] **Step 3: Replace nested route declarations with locked-version static APIs**

Change imports to:

```rust
use crate::posts::{post_slugs, post_tags, Post, PostSummary};
use leptos_router::{
  components::{FlatRoutes, Route, Router},
  path,
  static_routes::{StaticParamsMap, StaticRoute},
  SsrMode,
};
```

Replace the current `<Routes>` block with:

```rust
<FlatRoutes fallback=|| view! {
  <div class="container">
    <div class="not-found">
      <h1>"404"</h1>
      <p>"Page not found."</p>
      <a href="/" rel="external">"← Back to home"</a>
    </div>
  </div>
}.into_view()>
  <Route
    path=path!("/")
    view=HomePage
    ssr=SsrMode::Static(StaticRoute::new())
  />
  <Route
    path=path!("/archive")
    view=ArchivePage
    ssr=SsrMode::Static(StaticRoute::new())
  />
  <Route
    path=path!("/about")
    view=AboutPage
    ssr=SsrMode::Static(StaticRoute::new())
  />
  <Route
    path=path!("/posts/:slug")
    view=PostPage
    ssr=SsrMode::Static(StaticRoute::new().prerender_params(|| async {
      let posts = get_post_summaries().await.unwrap_or_default();
      [("slug".to_string(), post_slugs(&posts))]
        .into_iter()
        .collect::<StaticParamsMap>()
    }))
  />
  <Route
    path=path!("/tags/:tag")
    view=TagPage
    ssr=SsrMode::Static(StaticRoute::new().prerender_params(|| async {
      let posts = get_post_summaries().await.unwrap_or_default();
      [("tag".to_string(), post_tags(&posts))]
        .into_iter()
        .collect::<StaticParamsMap>()
    }))
  />
</FlatRoutes>
```

- [ ] **Step 4: Run the route test, formatter, and full Rust suite**

```bash
cargo fmt
cargo test --features ssr app::tests::all_public_routes_are_static -- --exact
cargo test --features ssr
```

Expected: all five routes report `SsrMode::Static`; the full suite passes.

- [ ] **Step 5: Commit Task 3**

```bash
git add src/app.rs
git commit -m "feat(ssg): mark blog routes static"
```

### Task 4: Generate complete static output without starting Axum

**Files:**
- Modify: `src/main.rs:1-110`
- Test: unit test inside `src/main.rs`

**Interfaces:**
- Consumes: `SITE_URL`, `load_posts`, the three parameterized generators
- Produces: `fn write_static_metadata(site_root: &Path, posts: &[Post], site_url: &str) -> io::Result<()>`
- Produces: SSG-only behavior controlled by presence of `LEPTOS_SSG_ONLY`

- [ ] **Step 1: Write a failing static metadata test**

Append to `src/main.rs`:

```rust
#[cfg(all(test, feature = "ssr"))]
mod tests {
  use super::*;
  use std::{fs, time::{SystemTime, UNIX_EPOCH}};

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
```

- [ ] **Step 2: Run the test and verify RED**

```bash
cargo test --features ssr --bin blog writes_static_metadata_files
```

Expected: compilation fails because `write_static_metadata` does not exist.

- [ ] **Step 3: Implement the standard-library writer**

Add above `main`:

```rust
#[cfg(feature = "ssr")]
fn write_static_metadata(
  site_root: &std::path::Path,
  posts: &[blog::posts::Post],
  site_url: &str,
) -> std::io::Result<()> {
  std::fs::create_dir_all(site_root)?;
  std::fs::write(site_root.join("rss.xml"), blog::rss::generate_rss(posts, site_url))?;
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
```

- [ ] **Step 4: Switch main to `generate_route_list_with_ssg` and add the SSG-only exit**

Use:

```rust
use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};

let (routes, static_routes) = generate_route_list_with_ssg({
  let leptos_options = leptos_options.clone();
  move || shell(leptos_options.clone())
});
static_routes.generate(&leptos_options).await;

if std::env::var_os("LEPTOS_SSG_ONLY").is_some() {
  let posts = load_posts();
  write_static_metadata(
    std::path::Path::new(&leptos_options.site_root),
    &posts,
    blog::SITE_URL,
  )
  .expect("failed to write static metadata");
  return;
}
```

Pass `blog::SITE_URL` to all three existing Axum metadata handlers so local SSR still compiles.

- [ ] **Step 5: Verify GREEN and generate the first complete static site**

```bash
cargo fmt
cargo test --features ssr --bin blog writes_static_metadata_files
cargo test --features ssr
SITE_URL=https://static-test.invalid cargo leptos build --release
LEPTOS_SSG_ONLY=1 ./target/release/blog
test -f target/site/index.html
test -f target/site/archive.html
test -f target/site/about.html
test -n "$(find target/site/posts -maxdepth 1 -name '*.html' -print -quit)"
test -n "$(find target/site/tags -maxdepth 1 -name '*.html' -print -quit)"
test -f target/site/rss.xml
test -f target/site/sitemap.xml
test -f target/site/robots.txt
```

Expected: tests pass, the binary exits without listening, and every file check succeeds.

- [ ] **Step 6: Commit Task 4**

```bash
git add src/main.rs
git commit -m "feat(ssg): emit complete static site"
```

### Task 5: Remove runtime server dependencies from rendered navigation

**Files:**
- Modify: `src/app.rs:140-705`
- Modify: `src/components/post_card.rs:14-40`
- Modify: `src/components/archive_page.rs:18-90`
- Modify: `src/components/about_page.rs:5-87`
- Modify: `docs/superpowers/specs/2026-08-28-cloudflare-pages-ssg-design.md`

**Interfaces:**
- Consumes: `crate::SITE_URL`
- Produces: generated HTML with the configured origin and no internal anchor that Leptos intercepts

- [ ] **Step 1: Verify the current static output is RED**

Run after Task 4 generated `target/site`:

```bash
rg -n 'abletonpilot\.onrender\.com' target/site --glob '*.html'
rg -n '<a [^>]*href=(format!\()?"/' src -g '*.rs' | rg -v 'rel="external"'
```

Expected: the first command finds hardcoded Render metadata and the second finds internal anchors without `rel="external"`.

- [ ] **Step 2: Replace rendered origins with `SITE_URL`**

Import `crate::SITE_URL` where needed. Delete the self-origin DNS-prefetch in `shell`. Use direct values for the homepage and local formatted strings for subpaths:

```rust
let about_url = format!("{SITE_URL}/about");
let post_url = format!("{SITE_URL}/posts/{}", post.slug);
```

Apply these values to canonical links, Open Graph URLs, Twitter URLs, JSON-LD author/publisher URLs, RSS, sitemap, and robots generation. Do not change third-party GitHub, LinkedIn, Ko-fi, Google, or Giscus URLs.

- [ ] **Step 3: Make every repository-internal anchor a document navigation**

Add `rel="external"` directly to each internal `<a>` in the four listed UI files:

```rust
<a href="/archive" rel="external">"Archive"</a>
<a href=format!("/posts/{}", slug) rel="external">{title}</a>
<a href=format!("/tags/{}", tag_link) class="tag" rel="external">{tag_text}</a>
```

Keep external links' existing `noopener noreferrer` values unchanged.

- [ ] **Step 4: Rebuild and verify GREEN**

```bash
cargo fmt
cargo test --features ssr
SITE_URL=https://static-test.invalid cargo leptos build --release
LEPTOS_SSG_ONLY=1 ./target/release/blog
if rg -n 'abletonpilot\.onrender\.com' target/site --glob '*.html'; then exit 1; fi
if rg -n '<a [^>]*href=(format!\()?"/' src -g '*.rs' | rg -v 'rel="external"'; then exit 1; fi
rg -n 'https://static-test\.invalid' target/site/index.html target/site/rss.xml target/site/sitemap.xml target/site/robots.txt
```

Expected: tests pass, no generated HTML contains the Render origin, no internal source anchor lacks `rel="external"`, and all four output types contain the configured test origin.

- [ ] **Step 5: Commit Task 5**

```bash
git add src/app.rs src/components/post_card.rs src/components/archive_page.rs src/components/about_page.rs docs/superpowers/specs/2026-08-28-cloudflare-pages-ssg-design.md
git commit -m "fix(ssg): remove runtime navigation calls"
```

### Task 6: Add the Cloudflare Pages Direct Upload workflow

**Files:**
- Create: `.github/workflows/deploy-pages.yml`

**Interfaces:**
- Consumes: GitHub variables `SITE_URL`, `CLOUDFLARE_PAGES_PROJECT`
- Consumes: GitHub secrets `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`
- Produces: verified `target/site` deployment using `cloudflare/wrangler-action@v4`

- [ ] **Step 1: Confirm the workflow is absent**

```bash
test ! -e .github/workflows/deploy-pages.yml
```

Expected: exit 0, proving the new workflow file does not already exist.

- [ ] **Step 2: Create the minimal deployment workflow**

Create `.github/workflows/deploy-pages.yml` with:

```yaml
name: Deploy Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  deployments: write

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: 1.90.0
          targets: wasm32-unknown-unknown
          components: rustfmt

      - uses: Swatinem/rust-cache@v2

      - name: Install cargo-leptos
        run: |
          curl --proto '=https' --tlsv1.2 -LsSf \
            https://github.com/leptos-rs/cargo-leptos/releases/download/v0.2.47/cargo-leptos-installer.sh | sh

      - name: Validate public variables
        env:
          SITE_URL: ${{ vars.SITE_URL }}
          PAGES_PROJECT: ${{ vars.CLOUDFLARE_PAGES_PROJECT }}
        run: |
          test -n "$SITE_URL" || { echo "::error::SITE_URL is not configured"; exit 1; }
          test -n "$PAGES_PROJECT" || { echo "::error::CLOUDFLARE_PAGES_PROJECT is not configured"; exit 1; }
          case "$SITE_URL" in
            https://*|http://*) ;;
            *) echo "::error::SITE_URL must start with http:// or https://"; exit 1 ;;
          esac
          case "$SITE_URL" in
            */) echo "::error::SITE_URL must not end with /"; exit 1 ;;
          esac

      - name: Format check
        run: cargo fmt --check

      - name: Test
        run: cargo test --features ssr

      - name: Build
        env:
          SITE_URL: ${{ vars.SITE_URL }}
        run: cargo leptos build --release

      - name: Generate static site
        env:
          LEPTOS_SSG_ONLY: "1"
        run: ./target/release/blog

      - name: Verify output
        run: |
          test -f target/site/index.html
          test -f target/site/archive.html
          test -f target/site/about.html
          test -n "$(find target/site/posts -maxdepth 1 -name '*.html' -print -quit)"
          test -n "$(find target/site/tags -maxdepth 1 -name '*.html' -print -quit)"
          test -f target/site/rss.xml
          test -f target/site/sitemap.xml
          test -f target/site/robots.txt

      - name: Deploy
        uses: cloudflare/wrangler-action@v4
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          command: pages deploy target/site --project-name=${{ vars.CLOUDFLARE_PAGES_PROJECT }}
          gitHubToken: ${{ secrets.GITHUB_TOKEN }}
```

- [ ] **Step 3: Validate the workflow's shell guards without revealing values**

Run:

```bash
env SITE_URL=https://static-test.invalid PAGES_PROJECT=blog sh -c '
  test -n "$SITE_URL" &&
  test -n "$PAGES_PROJECT" &&
  case "$SITE_URL" in https://*|http://*) ;; *) exit 1 ;; esac &&
  case "$SITE_URL" in */) exit 1 ;; esac
'
git diff --check
```

Expected: the shell guard and whitespace check exit 0.

- [ ] **Step 4: Commit Task 6**

```bash
git add .github/workflows/deploy-pages.yml
git commit -m "ci: deploy static site to Pages"
```

### Task 7: Run full static-site verification

**Files:**
- Verify only; no planned source edits

**Interfaces:**
- Consumes: all previous tasks
- Produces: fresh evidence that tests, SSG generation, extensionless route serving, metadata, and hydration work together

- [ ] **Step 1: Run all repository checks from a clean build**

```bash
cargo fmt --check
cargo test --features ssr
SITE_URL=https://static-test.invalid cargo leptos build --release
LEPTOS_SSG_ONLY=1 ./target/release/blog
git diff --check
```

Expected: every command exits 0.

- [ ] **Step 2: Verify generated files and content**

```bash
test -f target/site/index.html
test -f target/site/archive.html
test -f target/site/about.html
POST_FILE=$(find target/site/posts -maxdepth 1 -name '*.html' -print -quit)
TAG_FILE=$(find target/site/tags -maxdepth 1 -name '*.html' -print -quit)
test -n "$POST_FILE"
test -n "$TAG_FILE"
rg -n '<article class="post-detail"|class="post-content"' "$POST_FILE"
rg -n 'https://static-test\.invalid' target/site/index.html target/site/rss.xml target/site/sitemap.xml target/site/robots.txt
if rg -n 'abletonpilot\.onrender\.com' target/site --glob '*.html'; then exit 1; fi
```

Expected: fixed pages, at least one post and tag page, post HTML, and the configured origin are present; no generated HTML contains the Render origin.

- [ ] **Step 3: Serve through Cloudflare's local emulator**

Run in a persistent terminal:

```bash
npx --yes wrangler@4 pages dev target/site --port 4173
```

Then verify extensionless routing and static metadata:

```bash
curl -fsS http://127.0.0.1:4173/ >/dev/null
curl -fsS http://127.0.0.1:4173/archive >/dev/null
POST_PATH=$(find target/site/posts -maxdepth 1 -name '*.html' -print -quit | sed 's#^target/site/##; s#\.html$##')
test -n "$POST_PATH"
curl -fsS "http://127.0.0.1:4173/$POST_PATH" >/dev/null
curl -fsS http://127.0.0.1:4173/rss.xml | rg 'https://static-test\.invalid/rss.xml'
curl -fsS http://127.0.0.1:4173/sitemap.xml | rg 'https://static-test\.invalid/'
```

- [ ] **Step 4: Browser-smoke the hydrated UI**

Open `http://127.0.0.1:4173/` in Chromium and verify:

1. The home list renders without a loading placeholder remaining.
2. Search filters the visible post cards.
3. Theme toggle changes the page class.
4. Clicking a post performs a document request and renders `.post-detail` without any `/api/` failure.
5. The archive and about links render their pages.

Expected: all five checks pass with no console error from a missing server-function endpoint.

- [ ] **Step 5: Record final repository state**

```bash
git status --short
git log --oneline -7
```

Expected: only intentional commits are present and there are no uncommitted source changes. Do not push, create the Cloudflare project, change DNS, delete Render, or remove the keepalive workflow.
