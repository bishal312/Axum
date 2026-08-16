use axum::http::StatusCode;

pub async fn always_error() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
