mod hello_world;
mod mirror_body;
use axum::{Router, routing::{get, post}};

use hello_world::hello_world;

use crate::routes::mirror_body::mirror_body;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
    .route("/", post(mirror_body))
}