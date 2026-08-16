mod always_error;
mod hello_world;
mod middleware_message;
mod mirror_body;
mod mirror_body_json;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod return_201;
mod set_middleware_custom_header;
mod get_json;
mod validate_with_serde;
use crate::routes::{
    mirror_body::mirror_body, mirror_body_json::mirror_body_json, path_variables::hard_coded_path,
};
use always_error::always_error;
use axum::{
    Extension, Router,
    http::Method,
    middleware,
    routing::{get, post},
};
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use return_201::return_201;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use get_json::get_json;
use validate_with_serde::validate_with_serde;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data: SharedData = SharedData {
        message: "Hello from Shared Data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body", post(mirror_body))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/{id}", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_error", get(always_error))
        .route("/return_201", get(return_201))
        .route("/get_json", get(get_json))
        .route("/validate_with_serde", post(validate_with_serde))
}
