use std::{sync::Arc};

use axum::{Extension, Json, extract::Query, http::{HeaderMap, StatusCode, header}, response::{IntoResponse, Redirect}};
use axum_extra::extract::cookie::Cookie;
use chrono::{Utc, Duration};
use validator::Validate;

use crate::{AppState, db::UserExt, dtos::{ForgotPasswordRequestDto, LoginUserDto, RegisterUserDto, ResetPasswordRequestDto, Response, UserLoginResponseDto, VerifyEmailQueryDto }, error::{ErrorMessage, HttpError}, mail::mails::{send_forget_email, send_verification_email, send_welcome_email}, utils::{password, token}};

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
                status: "success".to_string(),
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


pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state.db_client
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request((ErrorMessage::WrongCredentials.to_string())))?;

    let password_matched = password::compare(&body.password, &user.password)
        .map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if password_matched {
        let token = token::create_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        )
        .map_err(|e| HttpError::server_error(e.to_string()))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
        let cookie = Cookie::build(("token", token.clone()))
            .path("/")
            .max_age(cookie_duration)
            .http_only(true)
            .build();

        let response = axum::response::Json(UserLoginResponseDto {
            status: "success".to_string(),
            token,
        });

        let mut header = HeaderMap::new();

        header.append(
            header::SET_COOKIE,
            cookie.to_string().parse().unwrap(),
        );

        let mut response = response.into_response();
        response.headers_mut().extend(header);

        Ok(response)
    } else {
        Err(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))
    }
}

// Step 1: Input & Request Extraction

// Receive the shared application state (app_state) and extract the JSON payload into LoginUserDto (body).

// Step 2: Input Validation

// Validate the incoming payload fields using body.validate().

// If validation fails: Map the validation error to an HttpError::server_error and terminate execution. (Note: Typically validation errors return 400 Bad Request, but here it returns server error).

// Step 3: User Lookup

// Asynchronously query the database via app_state.db_client.get_user() using the provided email address (&body.email).

// If the database query fails: Return an HttpError::server_error.

// If no user matches the email: Convert the None result into an HttpError::bad_request with a generic "Wrong Credentials" message.

// Step 4: Password Verification

// Compare the provided plain-text password (&body.password) against the hashed password stored in the user record (&user.password).

// If password comparison fails or errors out: Return an HttpError::bad_request with a "Wrong Credentials" message.

// Step 5: Authentication Token & Cookie Generation

// If the password matches:

// Generate a JWT token using the user's ID (user.id), the secret key (app_state.env.jwt_secret), and the maximum age setting. If token creation fails, return an HttpError::server_error.

// Calculate the cookie duration by converting jwt_maxage into minutes.

// Construct an HTTP-only token cookie scoped to path "/".

// Create a JSON response payload containing status: "success" and the generated JWT token.

// Build a HTTP Set-Cookie header containing the cookie string.

// Attach the Set-Cookie header to the response object and return Ok(response).

// If the password does not match: Return an HttpError::bad_request with a "Wrong Credentials" message.

pub async fn verify_email(
    Query(query_params): Query<VerifyEmailQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
    .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client
        .get_user(None, None, None, Some(&query_params.token))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(HttpError::bad_request("Verification token has been expired".to_string()))?;
        }
    }

    app_state.db_client.verified_token(&query_params.token).await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let send_welcome_mail_email_request = send_welcome_email(&user.email, &user.name).await;

    if let Err(e) = send_welcome_mail_email_request {
        eprintln!("Failed to send welcome email: {}", e);
    }

    let token = token::create_token(
        &user.id.to_string(),
        app_state.env.jwt_secret.as_bytes(),
        app_state.env.jwt_maxage,
    ).map_err(|e| HttpError::server_error(e.to_string()))?;

    let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    let mut headers = HeaderMap::new();

    headers.append(
        header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );

    let frontend_url = format!("http://localhost:5173/settings");

    let redirect = Redirect::to(&frontend_url);

    let mut response = redirect.into_response();

    response.headers_mut().extend(headers);

    Ok(response)
}

// Step 1: Extract Parameters & Validate Input

// Extract the query parameters (VerifyEmailQueryDto) and shared application state (AppState).

// Validate the query parameters (query_params.validate()).

// If validation fails: Return an HttpError::bad_request.

// Step 2: Database User Lookup

// Asynchronously query the database via get_user using the verification token (query_params.token).

// If database query fails: Return an HttpError::server_error.

// If no user matches the token: Return an HttpError::unauthorized with an "Invalid Token" message.

// Step 3: Expiration Check Logic (Contains Bug)

// Check if the user record has a token_expires_at timestamp:

// If expired (Utc::now() > expires_at): Return an HttpError::bad_request stating the token is expired.

// If valid/not expired: Return an HttpError::bad_request stating "Invalid verification token". (Note: This else block is a bug that prematurely blocks valid tokens).

// Step 4: Update Database Verification Status

// Call verified_token on the database client using the token to mark the user account as verified.

// If database update fails: Return an HttpError::server_error.

// Step 5: Dispatch Welcome Email

// Asynchronously call send_welcome_email using the user's email and name.

// If email dispatch fails: Print the failure error message to stderr (eprintln!) and continue execution without aborting.

// Step 6: Authentication Token & Cookie Generation

// Generate a JWT token using the user's ID (user.id), application secret key (jwt_secret), and duration settings (jwt_maxage).

// If token creation fails: Return an HttpError::server_error.

// Construct an HTTP-only token cookie scoped to path "/" with an expiration duration matching jwt_maxage.

// Create a HeaderMap and append the constructed Set-Cookie header.

// Step 7: Frontend Redirection Response

// Construct an HTTP redirect pointing to http://localhost:5173/settings.

// Attach the Set-Cookie header to the redirect response object.

// Return the complete HTTP redirect response (Ok(response)).


pub async fn forget_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ForgotPasswordRequestDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request("Email not found!".to_string()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state.db_client
        .add_verified_token(user_id, &verification_token, expires_at)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let reset_link = format!("http://localhost:5173/reset-password?token={}", &verification_token);

    let email_sent = send_forget_email(&user.email, &reset_link, &user.name).await;
    
    if let Err(e) = email_sent {
        eprint!("Failded to send forget password email reset link: {}", e);
        return Err(HttpError::server_error("Failed to send email". to_string()));
    }

    let response = Response {
        message: "Password reset link has been sent to your email.".to_string(),
        status: "success".to_string(),
    };

    Ok(Json(response))

}

pub async fn reset_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ResetPasswordRequestDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError:: bad_request(e.to_string()))?;

    let result = app_state.db_client
        .get_user(None, None, None, Some(&body.token))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::bad_request("Invalid or expired token".to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(HttpError::bad_request("Verification token has been expired".to_string()))?;
        }
    }
    
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let hash_password = password::hash(&body.new_password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    
    app_state.db_client
        .update_user_password(user_id.clone(), hash_password)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state.db_client
        .verified_token(&body.token)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = Response {
        message: "Password has been successfully reset.".to_string(),
        status: "success".to_string(),
    };

    Ok(Json(response))
}