use std::env;
use axum::routing::get;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct CreateSnap {
    message: String,
}

#[derive(Debug, serde::Serialize)]
struct SnapCreated {
    id: String,
    message: String,
}

#[derive(Debug, serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    data: T,
}

#[tokio::main]
pub async fn main() {
    let app = axum::Router::new()
        .fallback(
            fallback_handler
        )
        .route(
            "/snaps",
            get(snaps_get_handler)
                .post(snaps_post_handler)
        );

    let mut addr = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
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

async fn snaps_get_handler()
-> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "A list of snaps\n".to_string())
}

async fn snaps_post_handler(
    axum::extract::Json(payload): axum::extract::Json<CreateSnap>
) -> (axum::http::StatusCode, axum::extract::Json<ApiResponse<SnapCreated>>) {
    dbg!(&payload);
    let payload = SnapCreated {id: "abc".to_string(), message: payload.message};
    let response = ApiResponse { data: payload };
    (axum::http::StatusCode::CREATED, response.into())
}
