use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::env;

#[derive(Deserialize)]
struct UserPayload {
    name: String,
    email: String,
}

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to Database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/{id}",
            get(get_user)
                .put(update_user)
                .delete(delete_user),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    println!("Server running on port 8000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

// Test endpoint
async fn root() -> &'static str {
    "Welcome to the axum Website"
}

// GET ALL
async fn list_users(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users",
    )
    .fetch_all(&pool)
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// CREATE USER
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email)
         VALUES ($1, $2)
         RETURNING id, name, email",
    )
    .bind(payload.name)
    .bind(payload.email)
    .fetch_one(&pool)
    .await
    .map(|user| (StatusCode::CREATED, Json(user)))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// GET USER BY ID
async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email
         FROM users
         WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|_| StatusCode::NOT_FOUND)
}

// DELETE USER
async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM users WHERE id = $1",
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// UPDATE USER
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>(
        "UPDATE users
         SET name = $1, email = $2
         WHERE id = $3
         RETURNING id, name, email",
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|_| StatusCode::NOT_FOUND)
}