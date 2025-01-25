use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Users {
  pub id: i64,
  pub nama: String,
  pub email: String,
  pub sandi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
  pub nama: String,
  pub email: String,
  pub sandi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
  pub email: String,
  pub sandi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTExpire {
  pub id: String,
  pub exp: i64,
}