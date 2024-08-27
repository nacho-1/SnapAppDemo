use axum::routing::get;
use axum::extract::State;
use crate::state::SnapAppState;

#[derive(Debug, serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    data: T,
}

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
struct SnapInfo {
    id: String,
    message: String,
}

pub fn get_router<S: SnapAppState + Clone + Send + Sync + 'static>() -> axum::Router<S> {
    axum::Router::new()
        .fallback(
            fallback_handler
        )
        .route(
            "/snaps",
            get(snaps_get_handler::<S>)
                .post(snaps_post_handler::<S>)
        )
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
async fn fallback_handler(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// axum handler for "GET /snaps" which return a list
/// of snaps in JSON format.
async fn snaps_get_handler<S: SnapAppState>(
    State(repo): State<S>,
) -> (axum::http::StatusCode, axum::extract::Json<ApiResponse<Vec<SnapInfo>>>) {
    let snaps = repo.get()
        .iter()
        .map(|x|
            SnapInfo {
                id: x.id().clone(),
                message: x.message().clone()
            }
        )
        .collect::<Vec<SnapInfo>>();
    let response = ApiResponse { data: snaps };
    (axum::http::StatusCode::OK, response.into())
}

/// axum handler for "POST /snaps" which creates a new
/// snap in the repository. Will return some info on the
/// new snap alongside the status code.
async fn snaps_post_handler<S: SnapAppState>(
    State(mut repo): State<S>,
    axum::extract::Json(payload): axum::extract::Json<CreateSnap>,
) -> (axum::http::StatusCode, axum::extract::Json<ApiResponse<SnapCreated>>) {
    let snap = repo.post(payload.message).unwrap();
    let payload = SnapCreated {id: snap.id().clone(), message: snap.message().clone()};
    let response = ApiResponse { data: payload };
    (axum::http::StatusCode::CREATED, response.into())
}
