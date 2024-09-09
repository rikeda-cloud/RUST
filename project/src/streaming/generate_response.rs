use axum::body::Body;
use axum::http::StatusCode;
use axum::response::Response;

pub fn generate_not_found_response(error_message: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(error_message))
        .unwrap()
}

pub fn generate_text_response(body: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap()
}
