# Matrix Blog

A modern, sleek, dark-mode-native blog built with Rust. Drop Markdown files and they automatically become pages.

## Features

- **Rust Backend**: Axum + Tokio
- **Server-Side Rendering**: Manual HTML templates
- **Hot Reload**: Drop a .md file, see it instantly
- **Dark Mode Native**: Respects `prefers-color-scheme`
- **View Transitions**: Smooth page navigation
- **Accessibility**: Focus states, skip link, ARIA landmarks
- **Reduced Motion**: Respects `prefers-reduced-motion`
- **SEO Ready**: RSS feed, sitemap, robots.txt, OG/Twitter cards
- **Tag Support**: Clickable tags and tag index pages

## Quick Start

### Prerequisites

- Docker & Docker Compose
- (Optional) Rust 1.88+ for local development

### Development

```bash
# Build and run container
docker compose up --build

# Or via make
make build run
```

Access at http://localhost:8080

## Adding Content

Drop Markdown files into the `content/` directory:

```
content/posts/my-post.md   -> /posts/my-post
content/pages/about.md     -> /about
```

### Frontmatter Format

```yaml
---
title: My Post Title
date: 2024-01-15T10:00:00Z
tags: [tag1, tag2]
draft: false
description: Optional description
---

Your content here...
```

**Note**: Date must be in ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`).

### Markdown Tips

- **First H1 is automatic**: The post title from frontmatter is rendered as H1. Don't add another H1 at the start of your content - it will be automatically stripped.
- Use H2 (`##`) for sections within posts.

## Project Structure

```
matrix/
├── server/           # Rust project
│   ├── src/
│   │   ├── main.rs   # Entry point
│   │   ├── lib.rs    # Library root, logging init
│   │   ├── app.rs    # Router setup
│   │   ├── routes/   # HTTP handlers
│   │   │   ├── index.rs   # Home page
│   │   │   ├── posts.rs   # Post pages
│   │   │   ├── pages.rs   # Static pages
│   │   │   ├── seo.rs     # robots.txt, sitemap.xml, feed.xml
│   │   │   └── tags.rs    # Tag index and tag pages
│   │   ├── content/  # Markdown parsing, caching, loader
│   │   ├── templates/# HTML rendering (manual string building)
│   │   └── watcher.rs# File system monitoring (hot reload)
│   └── Cargo.toml
├── content/
│   ├── posts/        # Blog posts (.md)
│   └── pages/        # Pages (.md)
├── static/           # CSS, JS, assets
├── deploy/           # Production deployment files
│   ├── docker-compose.prod.yml
│   └── Caddyfile
├── docker-compose.yml
├── Dockerfile
└── .github/
    └── workflows/
        └── deploy.yml
```

## Routes

| Path | Handler |
|------|---------|
| `/` | List all posts |
| `/posts/{slug}` | Single post |
| `/about` | About page |
| `/tags` | All tags |
| `/tags/{tag}` | Posts by tag |
| `/feed.xml` | RSS feed (HTML page) |
| `/atom.xml` | RSS feed (XML for subscriptions) |
| `/sitemap.xml` | XML sitemap |
| `/robots.txt` | Robots.txt |
| `/static/*` | Static assets |

## Testing

```bash
# Run unit tests
cargo test --manifest-path server/Cargo.toml

# Or via make
make test

# Check formatting
cargo fmt --manifest-path server/Cargo.toml -- --check

# Run clippy lints
cargo clippy --manifest-path server/Cargo.toml -- -D warnings
```

## Production Deployment

The project automatically deploys to VPS via GitHub Actions on push to `main`.

### Access

- **Production URL**: https://your-domain.com
- **VPS**: your-vps-ip
- **Internal port**: 127.0.0.1:8080 (not publicly accessible)

### Architecture

```
GitHub Actions (push to main)
        │
        ▼
┌───────────────────┐     ┌──────────────────────────┐
│ Build & Push to   │     │    VPS                  │
│ GHCR              │────▶│  ┌────────────────────┐  │
└───────────────────┘     │  │ Caddy Reverse      │  │
                          │  │ Proxy (ports 80,443)│  │
                          │  └────────┬─────────┘  │
                          │           │            │
                          │  ┌────────▼─────────┐   │
                          │  │ matrix-blog     │   │
                          │  │ (port 8080)     │   │
                          │  └─────────────────┘   │
                          └─────────────────────────┘
```

### Manual Deployment Commands

```bash
# SSH to VPS
ssh your-user@your-vps-ip

# Check status
cd /docker/services/matrix
docker compose -f docker-compose.prod.yml ps

# View logs
docker compose -f docker-compose.prod.yml logs -f matrix-blog
docker compose -f docker-compose.prod.yml logs -f caddy-proxy

# Rollback to previous version
docker compose -f docker-compose.prod.yml pull ghcr.io/opusnano/matrix:sha-<sha>
docker compose -f docker-compose.prod.yml up -d
```

## Logging

Logs use structured logging via `tracing`:

```bash
# View logs
docker compose logs -f
```

## Customization

### CSS Variables

Edit `static/css/style.css` to customize colors, spacing, etc.

### Templates

Edit `server/src/templates/mod.rs` to change page layouts.

## License

MIT
