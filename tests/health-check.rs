use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use zerotoprod::app;

#[tokio::test]
async fn health_check() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health-check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        hyper::body::to_bytes(response.into_body())
            .await
            .unwrap()
            .len(),
        0
    );
}
