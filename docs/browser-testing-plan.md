# Browser Testing Plan

## Introduction
This document defines the approach for validating the blog across modern browsers. The scope covers critical user journeys, dark mode behavior, and responsive layout integrity.

## Test Environment
- Base URL: http://localhost:3000/
- Build Artifacts: `cargo leptos build --release`
- Data Set: Sample markdown posts located in the `posts/` directory

## Browser Coverage
| Platform | Browser | Version Guidance | Notes |
|----------|---------|------------------|-------|
| Desktop  | Chromium (Chrome or Edge) | Latest stable | Primary reference engine |
| Desktop  | Firefox | Latest stable | Validate keyboard navigation and accessibility |
| Desktop  | WebKit (Safari substitute via Playwright) | Latest stable | Focus on typography and layout quirks |
| Mobile   | Chromium emulation (Pixel 5) | Latest stable | Verify responsive layout, touch targets |
| Mobile   | WebKit emulation (iPhone 12) | Latest stable | Confirm dark mode rendering and spacing |

## Viewports
- Desktop: 1440 x 900, 1920 x 1080
- Tablet: 1024 x 768
- Mobile Portrait: 390 x 844
- Mobile Landscape: 844 x 390

## Test Scenarios
1. Load home page, verify hero content, pagination, and recent posts list.
2. Navigate to individual post detail via card click, confirm metadata, markdown rendering, syntax highlighting, and image display.
3. Use tag links to filter posts, ensure URL updates and content accuracy.
4. Execute client-side search to validate real-time results for title, content, and tags.
5. Trigger dark mode toggle across all pages, revalidate typography and contrast.
6. Attempt navigation to unknown slug and confirm 404 handling.
7. Inspect header and footer links for correctness and external targets.
8. Refresh cached pages to confirm state consistency between reloads.

## Procedure
1. Prepare production build artifacts.
2. Launch local server with production configuration.
3. For each browser and viewport combination:
   - Clear cache or run in private session.
   - Execute all scenarios in the order listed.
   - Capture screenshots for regressions or layout anomalies.
4. Log discoveries in the issue tracker template below.
5. Re-test resolved issues before closing.

## Issue Log Template
```
ID:
Browser:
Viewport:
Scenario:
Steps:
Expected:
Actual:
Artifacts:
Status:
```
