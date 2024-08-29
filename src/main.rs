use snap_app_demo::{router, state};

use std::env;

#[tokio::main]
pub async fn main() {
    let state = state::MockSnapRepository::new();
    let app = router::get_router().with_state(state);

    let mut addr = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    addr.insert_str(0, "0.0.0.0:");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

