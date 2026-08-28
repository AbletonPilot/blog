# Cloudflare Pages Static Deployment Design

## Goal

Deploy this Rust/Leptos personal blog to Cloudflare Pages without an always-on
server, idle shutdowns, or a framework rewrite.

## Chosen Approach

Use Leptos 0.8 static routing to prerender every public route during CI, then
upload `target/site` to a Cloudflare Pages Direct Upload project. Keep the
existing Axum server path for local development, but stop after prerendering
when `LEPTOS_SSG_ONLY` is set.

This is preferred over:

- Embedding every post in the browser WASM bundle, which adds code and bundle
  weight only to preserve client-side page transitions.
- Rewriting the blog with Zola, Hugo, or another static-site generator, which
  discards the existing Leptos UI.

## Architecture

```text
posts/*.md
    |
    v
GitHub Actions -- cargo leptos build -- Leptos static route generation
    |                                      |
    |                                      +-- /posts/<slug>/index.html
    |                                      +-- /tags/<tag>/index.html
    |                                      +-- rss.xml, sitemap.xml, robots.txt
    v
target/site -- Wrangler Direct Upload --> Cloudflare Pages CDN
```

Visitors receive prebuilt HTML, CSS, JavaScript, and WASM. No Rust process runs
for a request. WASM hydration retains search, pagination, theme controls, and
Giscus. Internal navigation uses normal document loads so it never calls a
missing server-function endpoint after deployment.

## Code Changes

### Static routes

Replace `Routes` with `FlatRoutes` because the repository has only flat route
definitions. Mark `/`, `/archive`, and `/about` with
`SsrMode::Static(StaticRoute::new())`.

Mark `/posts/:slug` and `/tags/:tag` static and supply `prerender_params`
computed from the existing post summaries:

- Slugs include every `PostSummary.slug`.
- Tags include the sorted, deduplicated union of every metadata tag.

The route parameter helpers live with post loading in `src/posts.rs`. They are
pure functions so they can be tested without starting Axum.

### Static generation process

In `src/main.rs`, replace `generate_route_list` with
`generate_route_list_with_ssg`, then call `static_routes.generate()` before
constructing the Axum router.

When `LEPTOS_SSG_ONLY` is set:

1. Generate all route HTML under the configured `site-root`.
2. Use the existing RSS, sitemap, and robots generators to write `rss.xml`,
   `sitemap.xml`, and `robots.txt` into the same directory.
3. Exit successfully without binding a TCP listener.

Without the variable, local `cargo leptos watch` behavior remains unchanged.

### Runtime-free navigation

Add `rel="external"` to repository-internal anchors. Leptos then allows the
browser to request the next prebuilt HTML document instead of performing a
client-side transition that could rerun a server function.

### Canonical site URL

Define one compile-time `SITE_URL` value in `src/lib.rs`:

```rust
pub const SITE_URL: &str = match option_env!("SITE_URL") {
  Some(value) => value,
  None => "https://abletonpilot.onrender.com",
};
```

Use it for canonical URLs, Open Graph metadata, JSON-LD, RSS, sitemap, and
robots output. Local builds retain the current URL. The deployment workflow
sets `SITE_URL` from the GitHub Actions variable `SITE_URL` so the production
HTML uses the final Pages or custom domain.

## Deployment Workflow

Add `.github/workflows/deploy-pages.yml`, triggered by pushes to `main` and
manual dispatch. It will:

1. Check out the repository.
2. Install Rust 1.90.0 and the `wasm32-unknown-unknown` target, matching the
   repository's working local toolchain.
3. Restore Cargo build caches.
4. Install `cargo-leptos` 0.2.47, matching the repository's working local
   toolchain.
5. Run the repository's Rust tests.
6. Build the release frontend and server binary with `SITE_URL` set at compile
   time.
7. Run the binary with `LEPTOS_SSG_ONLY=1`.
8. Verify expected static files exist.
9. Upload `target/site` with `cloudflare/wrangler-action`.

Required GitHub configuration:

- Secret `CLOUDFLARE_API_TOKEN`: token limited to Cloudflare Pages Edit.
- Secret `CLOUDFLARE_ACCOUNT_ID`: Cloudflare account identifier.
- Variable `CLOUDFLARE_PAGES_PROJECT`: existing Direct Upload project name.
- Variable `SITE_URL`: final origin, including `https://` and no trailing slash.

The workflow fails before deployment when either required variable is empty.
It does not create, delete, or reconfigure Cloudflare projects.

## Error Handling

- Static metadata writes return `io::Result`; any failure stops the SSG build.
- An unreadable or invalid Markdown file remains handled by the existing post
  loader and is excluded with its existing stderr diagnostic.
- Missing Cloudflare secrets fail at the deploy action without being printed.
- Missing public workflow variables fail in an explicit validation step.
- Render remains live until the Pages preview deployment passes verification.

## Tests and Verification

Use test-first changes for new Rust behavior:

- A unit test proves slug extraction preserves every post slug.
- A unit test proves tag extraction is sorted and deduplicated.
- A unit test writes static metadata to a unique standard-library temporary
  directory and asserts all three files contain the expected site URL.

Then run:

```text
cargo fmt --check
cargo test --features ssr
cargo leptos build --release
LEPTOS_SSG_ONLY=1 ./target/release/blog
```

Serve `target/site` locally and verify direct loads for `/`, `/archive`,
`/about`, one post, and one tag. Verify search, pagination, theme switching,
Giscus, `rss.xml`, `sitemap.xml`, and `robots.txt`. Inspect a generated post
file to confirm its body and metadata are present before JavaScript runs.

## Cutover

Deploy to the Pages preview origin first. After browser verification, connect
the final domain and set `SITE_URL` to that origin. Remove the Render keepalive
workflow only after the final origin is serving the verified deployment.

Cloudflare account setup, DNS changes, Render service deletion, and custom
404-page design are outside this code change.
