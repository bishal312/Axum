mod hello_world;
mod mirror_body;
mod mirror_body_json;
mod path_variables;
use axum::{Router, routing::{get, post}};

use hello_world::hello_world;

use path_variables::path_variables;

use crate::routes::{mirror_body::mirror_body, mirror_body_json::mirror_body_json, path_variables::hard_coded_path};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
    .route("/mirror_body", post(mirror_body))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/path_variables/{id}", get(path_variables))
    .route("/path_variables/15", get(hard_coded_path))
}