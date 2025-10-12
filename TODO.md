# Blog Development TODO

## Phase 1: Project Setup

- [x] Create posts directory structure
- [x] Add markdown parsing dependencies to Cargo.toml
  - pulldown-cmark for markdown parsing
  - serde and serde_yaml for front matter parsing
- [x] Set up basic post model and structure
- [x] Create sample markdown posts for testing

## Phase 2: Core Blog Features

### Post Management
- [x] Implement markdown file loader in src/posts.rs
- [x] Parse front matter (title, date, tags, description)
- [x] Convert markdown to HTML
- [x] Cache posts in memory on server startup
- [x] Sort posts by date (newest first)

### Routing and Pages
- [x] Create blog post list page
- [x] Create individual post detail page
- [x] Add dynamic routing for post slugs
- [x] Implement 404 page for missing posts

### UI Components
- [x] Design and implement blog layout
- [x] Create post list component
- [x] Create post detail component
- [x] Add navigation header
- [x] Add footer with links

## Phase 3: Content Features

- [x] Implement tag system
- [x] Create tag filter page
- [x] Add client-side search functionality
  - Search by post title
  - Search by post content
  - Search by tags
  - Display search results in real-time
- [ ] Implement pagination for post list
- [x] Add syntax highlighting for code blocks
- [x] Support for images in markdown posts

## Phase 4: Styling and UX

- [x] Design responsive layout
- [x] Implement dark mode toggle
- [x] Style markdown content (typography, spacing)
- [x] Add animations and transitions
- [x] Optimize mobile experience
- [ ] Test across different browsers

## Phase 5: SEO and Meta

- [ ] Add meta tags for each post
- [ ] Implement Open Graph tags
- [ ] Add Twitter Card tags
- [ ] Create sitemap generation
- [ ] Add RSS feed (optional)
- [ ] Optimize page load performance

## Phase 6: Deployment Preparation

### Build and Test
- [ ] Test production build locally
- [ ] Verify all routes work correctly
- [ ] Check mobile responsiveness
- [ ] Test markdown rendering edge cases
- [ ] Run end-to-end tests

### Deployment Configuration
- [ ] Choose deployment platform (Fly.io recommended)
- [ ] Create Dockerfile for containerization
- [ ] Set up environment variables
- [ ] Configure HTTPS and domain settings

## Phase 7: Deploy to Fly.io

- [ ] Install Fly.io CLI
- [ ] Create Fly.io account
- [ ] Initialize Fly.io app with flyctl launch
- [ ] Configure fly.toml settings
- [ ] Deploy with flyctl deploy
- [ ] Verify deployment at provided URL
- [ ] Set up custom domain (optional)

## Phase 8: Post-Deployment

- [ ] Set up monitoring and logging
- [ ] Create backup strategy for posts
- [ ] Document deployment process
- [ ] Set up CI/CD pipeline with GitHub Actions (optional)
- [ ] Plan content creation schedule

## Optional Enhancements

- [ ] Add related posts feature
- [ ] Create about page
- [ ] Add contact form
- [ ] Support multiple authors
- [ ] Add reading time estimate
- [ ] Create archive page by date
- [ ] Implement analytics (privacy-focused)

## Phase 9: GitHub Comments Integration (Giscus)

### Setup GitHub Discussions
- [ ] Enable GitHub Discussions in repository settings
- [ ] Create Announcements category for blog comments
- [ ] Configure discussion settings and permissions

### Configure Giscus
- [ ] Visit https://giscus.app
- [ ] Configure repository (AbletonPilot/blog)
- [ ] Select discussion category (Announcements)
- [ ] Choose mapping method (pathname recommended)
- [ ] Select theme (preferred_color_scheme for auto dark mode)
- [ ] Set language to Korean
- [ ] Copy generated script configuration

### Integrate into Leptos
- [ ] Create Giscus component in src/components/giscus.rs
- [ ] Add script loading logic for client-side rendering
- [ ] Include Giscus component in post detail page
- [ ] Test comments functionality in development
- [ ] Verify dark mode theme switching works
- [ ] Test on mobile devices

### Post-Integration
- [ ] Test commenting as different GitHub users
- [ ] Verify comments appear in GitHub Discussions
- [ ] Set up moderation guidelines if needed
- [ ] Document comment system for users

## Notes

- Keep markdown files in posts/ directory
- Use format: YYYY-MM-DD-slug.md for filenames
- Front matter format:
  ```yaml
  ---
  title: "Post Title"
  date: YYYY-MM-DD
  tags: [tag1, tag2]
  description: "Brief description"
  ---
  ```
- Commit posts to Git for version control
- Deploy updates by pushing to repository
