# Tezzembals — BALS Stack Template

A website template using the **BALS** stack for 3D games and web applications:

| Letter | Technology | Role |
|--------|------------|------|
| **B** | [Bevy](https://bevyengine.org/) | 3D game engine — rendering loop, ECS, physics |
| **A** | [Axum](https://github.com/tokio-rs/axum) | Web server — networking, WebSockets, API |
| **L** | [Leptos](https://leptos.dev/) | UI framework — DOM (menus, inventory) with no-GC signals |
| **S** | [SurrealDB](https://surrealdb.com/) | Database — Rust-native, graph-based game data |

## Prerequisites

- **Rust** (nightly) with `wasm32-unknown-unknown` target
- **cargo-leptos** — Leptos build tool
- **sass** — For SCSS compilation
- **nginx** — For production serving (optional)

```bash
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

## Project Structure

```
tezzembals/
├── app/           # Shared code (components, server functions)
├── frontend/      # Client WASM (Leptos + Bevy canvas)
├── server/        # Axum backend + SurrealDB
├── style/         # SCSS stylesheets
├── public/        # Static assets
└── nginx/         # Nginx configuration
```

## Quick Start

### Development (cargo-leptos)

```bash
cargo leptos watch
```

Opens http://127.0.0.1:3000 with hot-reload.

### Production Build

```bash
cargo leptos build --release
```

Output:
- **Server binary**: `target/server/release/server`
- **Site assets**: `target/site/` (HTML, JS, WASM, CSS)

### Run Server Only

```bash
cargo build -p server --release
./target/release/server
```

### With Nginx

1. Build the project: `cargo leptos build --release`
2. Copy `target/site/` to `/var/www/tezzembals/`
3. Start the Axum server on port 3000
4. Use the nginx config:

```bash
sudo cp nginx/nginx.conf /etc/nginx/sites-available/tezzembals
sudo ln -s /etc/nginx/sites-available/tezzembals /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

Update `root` in nginx.conf to your site path.

## Configuration

- **Leptos**: See `[workspace.metadata.leptos]` in `Cargo.toml`
- **SurrealDB**: In-memory by default. For production, use remote:

```rust
// server/src/main.rs - switch to remote
let db = Surreal::new::<Ws>("localhost:8000").await?;
```

Environment variables for remote SurrealDB:
- `SURREALDB_SERVER`, `SURREALDB_PORT`
- `SURREALDB_USERNAME`, `SURREALDB_PASSWORD`
- `SURREALDB_NS`, `SURREALDB_DB`

## Features

- **Bevy 3D** — Embedded via [leptos-bevy-canvas](https://github.com/Synphonyte/leptos-bevy-canvas)
- **Leptos UI** — Reactive inventory panel, menus
- **SurrealDB** — Graph-based data (in-memory template)
- **Axum** — SSR, API routes, static file serving

## License

MIT OR Apache-2.0
