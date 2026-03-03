# AGENTS.md - Developer Guidelines for Matrix Blog

This file provides guidelines and commands for agents working on this codebase.

---

## 1. Build, Lint, and Test Commands

### Docker

```bash
# Build the Docker image
docker compose build

# Run the container
docker compose up

# Run in background
docker compose up -d

# View logs
docker compose logs -f

# Stop the container
docker compose down

# Clean up all containers and volumes
docker compose down -v
```

### Rust Commands (inside container or with local Rust)

```bash
# Run all tests
cargo test --manifest-path server/Cargo.toml

# Run a single test by name
cargo test --manifest-path server/Cargo.toml test_frontmatter_parsing

# Run tests with output
cargo test --manifest-path server/Cargo.toml -- --nocapture

# Build release binary
cargo build --release --manifest-path server/Cargo.toml

# Check formatting
cargo fmt --manifest-path server/Cargo.toml -- --check

# Format code
cargo fmt --manifest-path server/Cargo.toml

# Run clippy lints (run before committing)
cargo clippy --manifest-path server/Cargo.toml -- -D warnings
```

**Important**: Always run `cargo fmt` and `cargo clippy` before committing.

---

## 2. Code Style Guidelines

### General Principles

- **Keep it simple**: Prefer readable, straightforward code over clever tricks
- **Explicit over implicit**: Make types and intentions clear
- **Fail fast**: Detect errors early at compile time rather than runtime
- **No useless code**: Avoid dead code, unused imports, and commented-out blocks

### Imports

- Use absolute paths for crate imports: `use crate::module::Item`
- Group imports in this order:
  1. Standard library (`std`, `core`)
  3. External crates (`axum`, `tokio`, etc.)
  4. Crate modules (`crate::`, `super::`)
- Use `use` for bringing items into scope; avoid `use module::*`

```rust
// Good
use std::sync::Arc;
use axum::{
    routing::get,
    extract::State,
    Router,
};
use crate::content::ContentLoader;

// Avoid
use crate::*;
use std::sync::*;
```

### Formatting

- Run `cargo fmt` before committing
- Use 4 spaces for indentation (Rust standard)
- Maximum line length: 100 characters (soft limit: 120)
- Add trailing comma in multi-line function calls when applicable

### Naming Conventions

- **Variables/functions**: `snake_case` (e.g., `handle_index`, `content_loader`)
- **Types/Traits**: `PascalCase` (e.g., `ContentLoader`, `Frontmatter`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_BUFFER_SIZE`)
- **Files**: `snake_case.rs` (e.g., `content_loader.rs`)

### Types

- Use explicit types in function signatures
- Prefer strong typing over primitive obsession
- Use `Arc<T>` for shared ownership across async contexts
- Use `&str` over `&String` unless you need ownership

```rust
// Good
pub async fn handle_index(
    State(loader): State<Arc<ContentLoader>>,
) -> Html<String> {
    let posts = loader.get_all_posts().await;
    Html(render_index(&posts))
}

// Avoid
pub async fn handle_index(state: State<Arc<ContentLoader>>) -> Html<String> {
    state.get_all_posts().await
}
```

### Error Handling

- Use `thiserror` for defining custom error types
- Use `?` operator for propagating errors
- Return meaningful error messages
- Never use `.unwrap()` in production code except where absolutely certain

```rust
// Good
pub enum ContentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

// In functions
fn load_content(path: &Path) -> Result<String, ContentError> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
```

### Async/Await

- Keep async functions small and focused
- Use `tokio` for async runtime
- Avoid blocking calls in async contexts
- Use `.await` immediately after async function calls

```rust
// Good
pub async fn get_posts(&self) -> Vec<Post> {
    if self.cache.read().posts.is_empty() {
        let _ = self.reload().await;
    }
    self.cache.read().posts.clone()
}
```

### Testing

- Write unit tests in the same module using `#[cfg(test)]`
- Name tests descriptively: `test_<function>_<expected_behavior>`
- Use descriptive assertion messages
- Test edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_parsing_with_valid_yaml() {
        let content = r#"---
title: Test
date: 2024-01-01T00:00:00Z
tags: [test]
draft: false
---

# Content"#;
        
        let (fm, _) = extract_frontmatter(content).unwrap();
        assert_eq!(fm.title, "Test");
    }
}
```

### Documentation

- Document public APIs with doc comments: `/// Description`
- Explain `why`, not `what`
- Keep comments updated; remove stale ones
- No comments on obvious code

### HTML/Templates

- When using manual string building, maintain consistent formatting
- Use proper HTML escaping (handled automatically by pulldown-cmark)
- Include semantic HTML elements for accessibility

### Git Conventions

- Use meaningful commit messages: "Add feature X" not "changes"
- Commit early, commit often
- Push to feature branches, not directly to main (when working with others)

---

## 3. Project Structure

```
server/
├── src/
│   ├── main.rs       # Entry point
│   ├── lib.rs        # Library root, logging init
│   ├── app.rs        # Router setup
│   ├── routes/       # HTTP handlers
│   │   ├── index.rs  # Home page
│   │   ├── posts.rs  # Post pages
│   │   ├── pages.rs  # Static pages
│   │   ├── seo.rs    # robots.txt, sitemap.xml, feed.xml
│   │   └── tags.rs   # Tag index and tag pages
│   ├── content/      # Markdown parsing, caching, loader
│   ├── templates/    # HTML rendering (manual string building)
│   └── watcher.rs   # File system monitoring (hot reload)
├── Cargo.toml
└── tests/            # Integration tests

content/
├── posts/            # Blog posts (.md)
└── pages/            # Pages (.md)

static/
├── css/
└── js/
```

---

## 4. Common Tasks

### Adding a new route

1. Add handler in `server/src/routes/`
2. Register in `server/src/app.rs`
3. Add template rendering in `server/src/templates/`

### Adding a new content type

1. Define struct in `server/src/content/parser.rs`
2. Add parsing logic
3. Add to cache if needed
4. Create template renderer

**Important**: When adding posts, frontmatter dates must use ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`), e.g., `2024-01-15T10:00:00Z`. Do not use `DD-MM-YYYY` format.

**Note**: The first H1 in post markdown is automatically stripped (title comes from frontmatter). Use H2 (`##`) for section headings in posts.

### Modifying CSS

Edit `static/css/style.css` - uses CSS variables for theming.

---

## 5. Key Dependencies

- **axum**: Web framework (0.8)
- **tokio**: Async runtime
- **pulldown-cmark**: Markdown parsing
- **notify**: File watching
- **tracing**: Structured logging
- **chrono**: Date/time handling

---

## 6. Troubleshooting

**Container won't start**: Check logs with `docker compose logs`

**Build fails**: Ensure Docker is running; try `docker compose build --no-cache`

**Tests fail**: Run with `-- --nocapture` to see output

**Port already in use**: Stop existing container or change port in compose file

### Testing SEO Endpoints

```bash
# Test new endpoints
curl http://localhost:8080/robots.txt
curl http://localhost:8080/sitemap.xml
curl http://localhost:8080/feed.xml
curl http://localhost:8080/atom.xml
curl http://localhost:8080/tags
curl http://localhost:8080/tags/rust

# Verify H1 is not duplicated in posts
curl http://localhost:8080/posts/hello-world | grep -c '<h1>'  # Should be 1

# Check cache headers on static assets
curl -I http://localhost:8080/static/css/style.css | grep cache-control
```

---

## 7. CI/CD Deployment

### GitHub Actions Workflow

The project uses GitHub Actions for automated deployment to VPS.

**Workflow file**: `.github/workflows/deploy.yml`

**Trigger**: Push to `main` branch

**What it does**:
1. Builds Docker image and pushes to GHCR (`ghcr.io/opusnano/matrix`)
2. SCPs deploy files to VPS (`deploy/docker-compose.prod.yml`, `deploy/Caddyfile`)
3. SSH into VPS, logs into GHCR, pulls image, runs `docker compose up -d`

### Deployment Files

| File | Purpose |
|------|---------|
| `deploy/docker-compose.prod.yml` | Production compose with matrix-blog + caddy-proxy |
| `deploy/Caddyfile` | Caddy reverse proxy config with Cloudflare origin cert |
| `.github/workflows/deploy.yml` | CI/CD workflow |

### VPS Deployment

```bash
# On VPS
ssh your-user@your-vps-ip
cd /docker/services/matrix

# Check containers
docker compose -f docker-compose.prod.yml ps

# View logs
docker compose -f docker-compose.prod.yml logs -f
docker compose -f docker-compose.prod.yml logs -f caddy-proxy
docker compose -f docker-compose.prod.yml logs -f matrix-blog

# Rollback to previous image
docker compose -f docker-compose.prod.yml pull ghcr.io/opusnano/matrix:sha-<commit-sha>
docker compose -f docker-compose.prod.yml up -d
```

### Secrets (GitHub Repository Settings)

| Secret | Description |
|--------|-------------|
| `VPS_HOST` | VPS IP address |
| `VPS_USER` | SSH username |
| `SSH_PRIVATE_KEY` | Private key for SSH authentication |
| `SSH_KEY_PASSPHRASE` | Passphrase for the SSH key |
| `GHCR_USERNAME` | GitHub username for GHCR login |
| `GHCR_TOKEN` | GitHub token with packages:write scope |
| `DOMAIN` | Production domain (e.g., example.com) - sets RUST_BASE_URL for canonical URLs |

### Adding a Second Site

1. Create new directory: `/docker/services/<site>`
2. Add compose file and Caddyfile in `deploy/`
3. Add new DNS record in Cloudflare
4. Update Caddyfile with new site block:
