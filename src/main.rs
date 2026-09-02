//! MineRStatus - binary entrypoint.
//!
//! Two build modes are selected by the `vercel` cargo feature (on by default):
//!
//! * **`vercel` (default)** - Vercel serverless function. The `vercel_runtime`
//!   bridge (`vercel_runtime::run`) communicates with Vercel's function host.
//!   Vercel's Rust framework detection hardcodes the entrypoint at
//!   `src/main.rs` and its builder runs `cargo build --bin main`, so this
//!   binary MUST be named `main`.
//! * **local (`cargo run --no-default-features`)** - a plain axum TCP server
//!   with no Vercel runtime linked.

#[cfg(feature = "vercel")]
mod entry {
    use minerstatus::{config, status::AppState};
    use tower::ServiceBuilder;
    use vercel_runtime::Error;
    use vercel_runtime::axum::VercelLayer;

    pub async fn run() -> Result<(), Error> {
        let cfg = config::load();
        let state = AppState::new(&cfg);
        let router = minerstatus::app::router(state);

        let app = ServiceBuilder::new()
            .layer(VercelLayer::new())
            .service(router);

        vercel_runtime::run(app).await
    }
}

#[cfg(not(feature = "vercel"))]
mod entry {
    use minerstatus::{app, config, status::AppState};

    pub async fn run() {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info,tower_http=info".into()),
            )
            .init();

        let cfg = config::load();
        tracing::info!(
            "cache_ttl=%{:?} cache_size={} timeout=%{:?}",
            cfg.cache.ttl,
            cfg.cache.max_size,
            cfg.query.timeout,
        );

        let addr = format!("{}:{}", cfg.server.host, cfg.server.port);
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(err) => {
                tracing::error!("failed to bind {addr}: {err}");
                std::process::exit(1);
            }
        };

        tracing::info!(
            "MineRStatus {} listening on http://{addr}",
            env!("CARGO_PKG_VERSION")
        );
        axum::serve(listener, app::router(AppState::new(&cfg)))
            .await
            .expect("server error");
    }
}

#[cfg(feature = "vercel")]
#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    entry::run().await
}

#[cfg(not(feature = "vercel"))]
#[tokio::main]
async fn main() {
    entry::run().await
}
