use actix_web::{web, HttpResponse, Responder};
use crate::models::{CreateUser, Users};
use crate::routes::verify::hash_password;
use sqlx::SqlitePool;

pub async fn get_users(koneksi: web::Data<SqlitePool>) -> impl Responder {
  let result: Result<Vec<Users>, sqlx::Error> = sqlx::query_as::<_, Users>("select * from users")
    .fetch_all(koneksi.get_ref())
    .await;

  match result {
    Ok(users) => HttpResponse::Ok().json(users),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

pub async fn buat_users(user: web::Json<CreateUser>, koneksi: web::Data<SqlitePool>) -> impl Responder {
  let hashing: String = match hash_password(&user.sandi) {
    Ok(hashed) => hashed,
    Err(_) => return HttpResponse::InternalServerError().finish(),
  };

  let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("insert into users (nama, email, sandi) values (?, ?, ?)")
    .bind(&user.nama)
    .bind(&user.email)
    .bind(hashing)
    .execute(koneksi.get_ref())
    .await;

  match result {
    Ok(_) => HttpResponse::Created().json(user.into_inner()),
    Err(_) => HttpResponse::InternalServerError().json("Gagal membuat user"),
  }
}