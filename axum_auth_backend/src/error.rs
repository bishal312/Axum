use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_string().to_owned()
    }
}

impl ErrorMessage {
    fn to_string(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Password is required".to_owned(),
            ErrorMessage::ExceededMaxPasswordLength(length) => {
                format!("Password must be at most {length} characters")
            }
            ErrorMessage::HashingError => "Error occurred while hashing password".to_owned(),
            ErrorMessage::InvalidToken => "Invalid token".to_owned(),
            ErrorMessage::ServerError => "Internal server error".to_owned(),
            ErrorMessage::WrongCredentials => "Wrong credentials".to_owned(),
            ErrorMessage::EmailExist => "Email already exists".to_owned(),
            ErrorMessage::UserNoLongerExist => "User no longer exists".to_owned(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_owned(),
            ErrorMessage::PermissionDenied => "Permission denied".to_owned(),
            ErrorMessage::UserNotAuthenticated => "User not authenticated".to_owned(),
        }
    }
}
