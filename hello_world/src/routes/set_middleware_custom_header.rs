use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::routes::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // let headers = request.headers();

    // 1. Deconstruct the request into parts and body
    let (mut parts, body) = request.into_parts();

    // let message = headers
    //     .get("message")
    //     .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    // 2. Read the header from parts.headers
    let message = parts
        .headers
        .get("message")
        .ok_or(StatusCode::BAD_REQUEST)?;

    // let message = message
    //     .to_str()
    //     .map_err(|_error| StatusCode::BAD_REQUEST)?
    //     .to_owned();

    let message_str = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();

    // let extensions = request.extensions_mut();
    // extensions.insert(HeaderMessage(message));

    // 3. Mutate parts.extensions safely
    parts.extensions.insert(HeaderMessage(message_str));

    // 4. Reconstruct the request
    let request = Request::from_parts(parts, body);
    Ok(next.run(request).await)
}
