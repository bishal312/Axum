mod hello_world;
mod mirror_body;
mod mirror_body_json;
use axum::{Router, routing::{get, post}};

use hello_world::hello_world;

use crate::routes::{mirror_body::mirror_body, mirror_body_json::mirror_body_json};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
    .route("/mirror_body", post(mirror_body))
    .route("/mirror_body_json", post(mirror_body_json))
}