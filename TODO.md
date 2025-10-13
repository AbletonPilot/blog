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
  - [x] Search by post title
  - [x] Search by post content
  - [x] Search by tags
  - [x] Display search results in real-time
- [x] Implement pagination for posts (10 posts per page)
- [x] Add syntax highlighting for code blocks
- [x] Support for images in markdown posts

## Phase 4: Styling and UX

- [x] Design responsive layout
- [x] Implement dark mode toggle
- [x] Style markdown content (typography, spacing)
- [x] Add animations and transitions
- [x] Optimize mobile experience
- [x] Test across different browsers *(see docs/browser-testing-plan.md)*

## Phase 5: SEO and Meta

- [x] Add meta tags for each post
- [x] Implement Open Graph tags
- [x] Add Twitter Card tags
- [x] Create sitemap generation
- [x] Add RSS feed (optional)
- [x] Optimize page load performance

## Phase 6: Deployment Preparation

### Build and Test
- [x] Test production build locally
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

## Phase 9: GitHub Comments Integration (Giscus) ✅

### Setup GitHub Discussions
- [x] Enable GitHub Discussions in repository settings
- [x] Create Announcements category for blog comments
- [x] Configure discussion settings and permissions
- [x] Change to General category for public commenting

### Configure Giscus
- [x] Visit https://giscus.app
- [x] Configure repository (AbletonPilot/blog)
- [x] Select discussion category (General)
- [x] Choose mapping method (pathname)
- [x] Select theme (noborder_light/noborder_dark)
- [x] Set language to English
- [x] Copy generated script configuration
- [x] Disable reactions display

### Integrate into Leptos
- [x] Create Giscus component in src/components/giscus.rs
- [x] Add script loading logic for client-side rendering
- [x] Include Giscus component in post detail page
- [x] Update repo-id and category-id with actual values
- [x] Fix component mounting on page navigation
- [x] Set language to English
- [x] Implement dynamic theme switching (dark/light mode)
- [x] Add MutationObserver for theme change detection
- [x] Fix deprecated web-sys API warnings
- [x] Remove debug logging
- [x] Test comments functionality in development
- [x] Verify dark mode theme switching works
- [x] Test on mobile devices

### Post-Integration
- [x] Test commenting as different GitHub users
- [x] Verify comments appear in GitHub Discussions
- [ ] Set up moderation guidelines if needed
- [ ] Document comment system for users

## Phase 10: UI/UX Enhancements ✅

### Homepage Header Redesign
- [x] Add welcome message with emoji animation
- [x] Implement gradient title effect
- [x] Add social media icon links (GitHub, LinkedIn, Ko-fi)
- [x] Center-align header layout
- [x] Add hover effects to social icons (color + glow, no motion)
- [x] Optimize icon sizes and alignment

### Global Search Improvements
- [x] Move search from HomePage to SiteHeader
- [x] Create global SearchContext for state management
- [x] Auto-navigate to homepage when searching from other pages
- [x] Maintain search state across navigation

### RSS/Footer Improvements
- [x] Move RSS/Sitemap/Robots to Axum backend routes
- [x] Add proper Content-Type headers (application/rss+xml, etc.)
- [x] Remove RSS from footer (keep link in HTML head)
- [x] Add Ko-fi donation link to footer
- [x] Simplify footer to single-line layout

### Archive Page
- [x] Fix archive icon size alignment (36px desktop, 28px mobile)

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
