# AbletonPilot

A personal blog built with Rust and Leptos, featuring markdown-based posts, dark/light mode, real-time search, and GitHub Discussions comments.

🌐 **Live Site**: [https://abletonpilot.onrender.com](https://abletonpilot.onrender.com)

---

## 🚀 Features

- **Markdown-based Blog**: Write posts in `posts/` folder with YAML front matter
- **Dark/Light Mode**: System preference detection with localStorage persistence
- **Real-time Search**: Search by title, description, and tags
- **Tag System**: Filter posts by tags
- **Comments**: GitHub Discussions integration via Giscus
- **RSS Feed**: Available at `/rss.xml`
- **Responsive Design**: Mobile-optimized UI with hamburger menu
- **Syntax Highlighting**: Code blocks with multiple language support
- **SEO Optimized**: Meta tags, Open Graph, Twitter Cards, sitemap

---

## 🛠️ Tech Stack

- **Framework**: [Leptos 0.8](https://leptos.dev/) (Rust)
- **Server**: [Axum](https://github.com/tokio-rs/axum)
- **Styling**: SCSS with CSS custom properties
- **Markdown**: pulldown-cmark with syntect highlighting
- **Comments**: [Giscus](https://giscus.app/) (GitHub Discussions)
- **Deployment**: [Render.com](https://render.com/) (Free tier)

---

## 📝 Writing Posts

### 1. Create Markdown File

```bash
posts/YYYY-MM-DD-slug.md
```

### 2. Add Front Matter

```yaml
---
title: "Your Post Title"
date: YYYY-MM-DD
tags: [tag1, tag2]
description: "Brief description for SEO and search"
---
```

### 3. Write Content

Use standard Markdown syntax. See [docs/markdown-guide.md](docs/markdown-guide.md) for comprehensive guide.

### 4. Add Media (Optional)

```bash
public/YYYY-MM-DD/image.png
public/YYYY-MM-DD/video.mp4
```

Reference in markdown:
```markdown
![Image](/YYYY-MM-DD/image.png)
<video controls><source src="/YYYY-MM-DD/video.mp4" type="video/mp4"></video>
```

---

## 🏃 Local Development

### Prerequisites

- Rust (nightly)
- cargo-leptos
- WASM target

### Installation

```bash
# Install Rust nightly
rustup toolchain install nightly

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install cargo-leptos
```

### Run Development Server

```bash
cargo leptos watch
```

Visit `http://localhost:3000`

---

## 🚢 Deployment

This blog is deployed on [Render.com](https://render.com/) with automatic deployment on `git push`.

### Render Configuration

**Build Command:**
```bash
rustup target add wasm32-unknown-unknown && cargo install cargo-leptos && cargo leptos build --release
```

**Start Command:**
```bash
./target/release/blog
```

**Environment Variables:**
- `LEPTOS_OUTPUT_NAME` = `blog`
- `LEPTOS_SITE_ROOT` = `target/site`
- `LEPTOS_SITE_PKG_DIR` = `pkg`
- `LEPTOS_SITE_ADDR` = `0.0.0.0:10000`

### Deployment Workflow

1. Write post in `posts/`
2. Add media to `public/` (if needed)
3. Test locally: `cargo leptos watch`
4. Commit and push:
   ```bash
   git add .
   git commit -m "Add new post: title"
   git push origin main
   ```
5. Render automatically builds and deploys (2-5 minutes)

---

## 📂 Project Structure

```
blog/
├── src/
│   ├── app.rs              # Main application with routing
│   ├── main.rs             # Server entry point
│   ├── posts.rs            # Markdown parser and loader
│   ├── rss.rs              # RSS feed generator
│   ├── sitemap.rs          # Sitemap generator
│   └── components/         # Leptos components
├── posts/                  # Markdown blog posts
│   └── YYYY-MM-DD-slug.md
├── public/                 # Static assets (images, videos)
│   └── YYYY-MM-DD/
├── style/
│   └── main.scss           # Styles with dark/light themes
├── docs/
│   └── markdown-guide.md   # Markdown writing guide
└── Cargo.toml              # Dependencies and Leptos config
```

---

## 🎨 Customization

### Change Theme Colors

Edit `style/main.scss`:

```scss
:root {
  --bg-primary: #1a1a1a;
  --text-primary: #e8e8e8;
  --accent: #7ee787;
  // ...
}
```

### Update Site Metadata

Edit `src/app.rs`:

```rust
<Title text="Your Blog Name"/>
<Meta property="og:site_name" content="Your Blog Name"/>
```

### Modify Giscus Settings

Edit `src/components/giscus.rs` with your repository settings.

---

## 📊 Performance & Limits

**Render.com Free Tier:**
- 512MB RAM
- 0.1 CPU
- 750 hours/month
- Sleeps after 15min inactivity

**Storage:**
- Slug size limit: 500MB
- Current size: ~46MB
- Estimated capacity: 30-70 blog posts (with images/videos)

**Optimization Tips:**
- Compress images (target: 200-500KB)
- Use external CDN for large media (Cloudinary, YouTube)
- Enable UptimeRobot to prevent sleep (optional)

---

## 📖 Documentation

- [Markdown Guide](docs/markdown-guide.md) - Comprehensive writing guide
- [Giscus Setup](docs/giscus-setup-guide.md) - Comments integration
- [TODO](TODO.md) - Development progress and features

---

## 🤝 Contributing

This is a personal blog, but feel free to:
- Report issues
- Suggest features
- Fork for your own use

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- **Blog**: [https://abletonpilot.onrender.com](https://abletonpilot.onrender.com)
- **GitHub**: [https://github.com/AbletonPilot](https://github.com/AbletonPilot)
- **Ko-fi**: [https://ko-fi.com/abletonpilot](https://ko-fi.com/abletonpilot)

---

**Built with ❤️ using Rust and Leptos**
