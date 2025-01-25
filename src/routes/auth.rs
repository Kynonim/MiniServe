use actix_web::{web, HttpResponse, Responder};
use crate::models::{Users, LoginUser};
use crate::routes::verify::{buat_token_jwt, verify_password};
use sqlx::SqlitePool;

pub async fn login(user: web::Json<LoginUser>, koneksi: web::Data<SqlitePool>) -> impl Responder {
  let result = sqlx::query_as::<_, Users>("select * from users where email = ?")
    .bind(&user.email)
    .fetch_optional(koneksi.get_ref())
    .await;
  
  match result {
    Ok(Some(person)) => {
      if verify_password(&user.sandi, &person.sandi) {
        match buat_token_jwt(person.id) {
          Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
          Err(_) => HttpResponse::InternalServerError().finish(),
        }
      } else {
        HttpResponse::Unauthorized().json("Email atau sandi salah")
      }
    }
    _ => HttpResponse::NotFound().json("Email atau sandi salah")
  }
}