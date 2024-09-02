use axum::{
    routing,
    extract::{
        State,
        Json,
        rejection::JsonRejection
    },
    http::{header, StatusCode},
    response::IntoResponse,
};
use crate::state::{SnapAppState, SnapCreationError};

#[derive(Debug, serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    data: T,
}

#[derive(Debug, serde::Serialize)]
struct ProblemResponse {
    #[serde(rename = "type")]
    uri: Option<String>,
    title: Option<String>,
    status: Option<String>,
    detail: Option<String>,
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

/// Instantiate a router for the app needing a state of type [S].
/// To use it, the method [axum::Router<S>::with_state] must be called on it.
pub fn get_router<S: SnapAppState + Clone + Send + Sync + 'static>() -> axum::Router<S> {
    axum::Router::new()
        .fallback(
            fallback_handler
        )
        .route(
            "/snaps",
            routing::get(snaps_get_handler::<S>)
                .post(snaps_post_handler::<S>)
        )
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
async fn fallback_handler(
    uri: axum::http::Uri
) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// axum handler for "GET /snaps" which return a list
/// of snaps in JSON format.
async fn snaps_get_handler<S: SnapAppState>(
    State(repo): State<S>,
) -> impl IntoResponse {
    let snaps = repo.get()
        .iter()
        .map(|x|
            SnapInfo {
                id: x.id(),
                message: x.message().to_string()
            }
        )
        .collect::<Vec<SnapInfo>>();
    let response = ApiResponse { data: snaps };
    (StatusCode::OK, Json::from(response))
}

/// axum handler for "POST /snaps" which creates a new
/// snap in the repository. Will return some info on the
/// new snap alongside the status code.
async fn snaps_post_handler<S: SnapAppState>(
    State(mut repo): State<S>,
    extractor: Result<Json<CreateSnap>, JsonRejection>,
) -> impl IntoResponse {
    match extractor {
        Ok(Json(payload)) => {
            match repo.post(&payload.message)  {
                Ok(snap) => {
                    let payload = SnapCreated {
                        id: snap.id(),
                        message: snap.message().to_string()
                    };
                    let response = ApiResponse { data: payload };
                    (StatusCode::CREATED, Json::from(response)).into_response()
                },
                Err(e) => {
                    map_snap_creation_error(e).into_response()
                }
            }
        },
        Err(rejection) => {
            handle_bad_json(&rejection).into_response()
        }
    }
}

/// Parse [SnapConnectionError] into an RFC 7807 compliant error response.
fn map_snap_creation_error(_error: SnapCreationError) -> impl IntoResponse {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let response = ProblemResponse {
        uri: None,
        title: Some("Unknown error".to_string()),
        status: Some(status.to_string()),
        detail: Some("Can't determine error cause".to_string())
    };
    (
        status,
        [(header::CONTENT_TYPE, "application/problem+json")],
        Json::from(response)
    )
}

/// Parse [JsonRejection] into an RFC 7807 compliant error response.
fn handle_bad_json(rejection: &JsonRejection) -> impl IntoResponse {
    let status = StatusCode::BAD_REQUEST;
    let response = ProblemResponse {
        uri: None,
        title: Some("Problem Parsing Json".to_string()),
        status: Some(status.to_string()),
        detail: Some(rejection.body_text()),
    };
    (
        status,
        [(header::CONTENT_TYPE, "application/problem+json")],
        Json::from(response),
    )
}
