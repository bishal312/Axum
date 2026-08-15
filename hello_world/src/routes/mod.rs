mod hello_world;
mod mirror_body;
mod mirror_body_json;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
use axum::{
    Router,
    routing::{get, post},
};

use hello_world::hello_world;

use path_variables::path_variables;

use query_params::query_params;

use crate::routes::{
    mirror_body::mirror_body, mirror_body_json::mirror_body_json, path_variables::hard_coded_path,
};

use mirror_user_agent::mirror_user_agent;

use mirror_custom_header::mirror_custom_header;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body", post(mirror_body))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/{id}", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
}
