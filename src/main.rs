use snap_app_demo::{router, state};
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    util::SubscriberInitExt,
    layer::SubscriberExt,
};

#[tokio::main]
pub async fn main() {
    // Subscriber for logs.
    // See: https://docs.rs/tracing-core/0.1.32/tracing_core/subscriber/trait.Subscriber.html
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = state::MockSnapRepository::new();
    let app = router::get_router()
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let mut addr = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    addr.insert_str(0, "0.0.0.0:");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    tracing::debug!("LISTENING ON {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
