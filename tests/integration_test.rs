use snap_app_demo::{router, state};
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
};
use serde_json::{json, Value};
use tower::ServiceExt;
use http_body_util::BodyExt;
use tokio::net::TcpListener;

#[tokio::test]
async fn post_snap_logic() {
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
        ).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["data"]["message"], json!("Test Snap"));
}

#[tokio::test]
async fn post_snap() {
    let state = state::MockSnapRepository::new();
    let app = router::get_router().with_state(state);

    let listener = TcpListener::bind("localhost:0")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    // Run the server in a thread.
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Create a client to communicate with the server.
    let client =
        hyper_util::client::legacy::Builder::new(hyper_util::rt::TokioExecutor::new())
            .build_http();

    let response = client
        .request(
            Request::builder()
                .method("POST")
                .uri(format!("http://{addr}/snaps"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "message": "Test Snap",
                    })).unwrap()
                ))
                .unwrap(),
        ).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["data"]["message"], json!("Test Snap"));
}

#[tokio::test]
async fn get_snaps() {
    let state = state::MockSnapRepository::new();
    let app = router::get_router().with_state(state);

    let listener = TcpListener::bind("localhost:0")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    // Run the server in a thread.
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Create a client to communicate with the server.
    let client =
        hyper_util::client::legacy::Builder::new(hyper_util::rt::TokioExecutor::new())
        .build_http();

    // Post 2 snaps.
    client.request(
        Request::builder()
            .method("POST")
            .uri(format!("http://{addr}/snaps"))
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&json!({
                        "message": "Test Snap 1",
                    })).unwrap()
            ))
            .unwrap(),
    ).await.unwrap();
    client.request(
        Request::builder()
            .method("POST")
            .uri(format!("http://{addr}/snaps"))
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&json!({
                        "message": "Test Snap 2",
                    })).unwrap()
            ))
            .unwrap(),
    ).await.unwrap();

    // "GET" request for the 2 snaps posted.
    let response = client
        .request(
            Request::builder()
                .method("GET")
                .uri(format!("http://{addr}/snaps"))
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
    // message "Test Snap 2" got posted last so it should come first.
    assert_eq!(body["data"][0]["message"], json!("Test Snap 2"));
    assert_eq!(body["data"][1]["message"], json!("Test Snap 1"));
}
