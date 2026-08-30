use std::{result, sync::Arc};

use axum::{Extension, Json, body, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use time::Duration;
use validator::Validate;

use crate::{AppState, db::UserExt, dtos::RegisterUserDto, error::{ErrorMessage, HttpError}, mail::mails::send_verification_email, utils::password};

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);

    let hash_password = password::hash(&body.password)
            .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state.db_client
        .save_user(&body.name, &body.email, &body.password, &verification_token, expires_at)
        .await;

    match result {
        Ok(_user) => {
            let send_email_result = send_verification_email(&body.email, &body.name, &verification_token).await;

            if let Err(e) = send_email_result {
                eprintln!("Failed to send verification email: {}", e);
            }

            Ok((StatusCode::CREATED, Json(Response{
                status: "success",
                message: "Registeration successfull! Please check your email to verify your account.".to_string(),
            })))
        },
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::EmailExist.to_string(),
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        },
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

// Step 1: Input & Request Handling

// Receive the application state (app_state) and the request payload mapped to RegisterUserDto (body).

// Step 2: Input Validation

// Validate the incoming user data (body.validate()).

// If validation fails: Map the validation errors to HttpError::bad_request and immediately return the error response.

// Step 3: Security & Expiration Token Generation

// Generate a new unique verification token using Uuid::new_v4().

// Calculate the expiration timestamp (expires_at) by adding 24 hours to the current UTC time.

// Step 4: Password Hashing

// Hash the plain text password (body.password).

// If hashing fails: Return an HttpError::server_error.

// Step 5: Database Persistence

// Asynchronously call save_user on the database client passing user details, hashed password, verification token, and expiration time.

// Step 6: Handle Database Operation Result

// Case A: Success (Ok(_user))

// Call send_verification_email asynchronously to dispatch the verification email.

// If sending the email fails, log the error to the console (eprintln!).

// Return an HTTP 201 CREATED status code alongside a JSON response payload indicating successful registration.

// Case B: Database Error (Err(sqlx::Error::Database(db_err)))

// Check if the database error is a unique constraint violation (e.g., duplicate email).

// If unique constraint violated: Return an HttpError::unique_constraint_violation with an "Email already exists" message.

// Otherwise: Return an HttpError::server_error with the database error message.

// Case C: Other Errors (Err(e))

// Return a generic HttpError::server_error containing the error details.
