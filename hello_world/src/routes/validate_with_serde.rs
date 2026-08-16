use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser{
    username: String,
    password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) -> Json<RequestUser>{
    dbg!(Json(user))
}