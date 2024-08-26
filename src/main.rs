use std::env;

mod router;

#[tokio::main]
pub async fn main() {
    let app = router::get_router();

    let mut addr = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    addr.insert_str(0, "0.0.0.0:");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

