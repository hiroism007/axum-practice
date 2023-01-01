use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}
