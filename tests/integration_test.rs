use axum::body::Body;
use axum::extract::Request;
use axum::handler::Handler;
use axum::http::StatusCode;
use snap_app_demo::{router, state};
use serde_json::{json, Value};
use tower::{
    Service,
    ServiceExt,
};
use http_body_util::BodyExt;


#[tokio::test]
async fn post_snap() {
    let state = state::MockSnapRepository::new();
    let app = router::get_router().with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/snaps")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "message": "Test Snap",
                    })).unwrap()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["data"]["message"], json!("Test Snap"));
}
