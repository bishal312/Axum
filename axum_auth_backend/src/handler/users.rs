use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};

use crate::{AppState, dtos::{FilterUserDto, UserData, UserResponseDto}, error::HttpError, middleware::JWTAUTHMiddleware};



pub async fn get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAUTHMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user.user);
    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: filtered_user,
        }
    };

    Ok(Json(response_data))
}
