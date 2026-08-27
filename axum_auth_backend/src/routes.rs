use std::sync::Arc;

use axum::{ middleware, Extension, Router };
use tower_http::trace::TraceLayer;
