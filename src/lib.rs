use axum::{http::StatusCode, routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/health-check", get(|| async { StatusCode::OK }))
}
