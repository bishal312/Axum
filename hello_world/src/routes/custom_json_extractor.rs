use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize,Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

//Mo #[async_trait] attribute required
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = Json::<RequestUser>::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

        if user.username.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Username cannot be empty".into()));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> (StatusCode, String) {
    (
        StatusCode:: OK,
        format!("Welcome, {}! Authentication successful.", user.username),
    )
}
