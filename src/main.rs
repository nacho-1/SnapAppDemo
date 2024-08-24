use std::env;

#[tokio::main]
pub async fn main() {
    let app = axum::Router::new()
        .fallback(
            fallback_handler
        );

    let mut addr = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    addr.insert_str(0, "0.0.0.0:");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
async fn fallback_handler(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}
