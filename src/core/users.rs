use actix_web::{web, HttpResponse, Responder};
use crate::models::{CreateUser, LoginUser, Users};
use crate::routes::verify::{buat_token_jwt, hash_password, verify_password};
use sqlx::{Row, SqlitePool};

pub async fn get_users(koneksi: web::Data<SqlitePool>) -> impl Responder {
  let result: Result<Vec<Users>, sqlx::Error> = sqlx::query_as::<_, Users>("select * from users")
    .fetch_all(koneksi.get_ref())
    .await;

  match result {
    Ok(users) => HttpResponse::Ok().json(users),
    Err(_) => HttpResponse::BadRequest().json(serde_json::json!({"status": false, "message": "Gagal mendapatkan data users"})),
  }
}

pub async fn buat_users(user: web::Json<CreateUser>, koneksi: web::Data<SqlitePool>) -> impl Responder {
  println!("{}", user.email);
  let cek_user = sqlx::query("select count (*) from users where email = ?")
    .bind(&user.email)
    .fetch_one(koneksi.get_ref())
    .await;
  match cek_user {
    Ok(row) => {
      let count: i64 = row.get(0);
      if count > 0 {
        return HttpResponse::Conflict().json(serde_json::json!({"status": false, "message": "Email sudah terdaftar"}));
      }
    }
    Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({"status": false, "message": "Gagal memeriksa email"}))
  }

  let hashing: String = match hash_password(&user.sandi) {
    Ok(hashed) => hashed,
    Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({"status": false, "message": "Gagal menghash password"})),
  };
  let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("insert into users (nama, email, sandi) values (?, ?, ?)")
    .bind(&user.nama)
    .bind(&user.email)
    .bind(hashing)
    .execute(koneksi.get_ref())
    .await;
  match result {
    Ok(_) => HttpResponse::Created().json(serde_json::json!({"status": true, "message": "Berhasil membuat akun"})),
    Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"status": false, "message": "Gagal membuat user"})),
  }
}

pub async fn login_users(user: web::Json<LoginUser>, koneksi: web::Data<SqlitePool>) -> impl Responder {
  let result: Result<Option<Users>, sqlx::Error> = sqlx::query_as::<_, Users>("select * from users where email = ?")
    .bind(&user.email)
    .fetch_optional(koneksi.get_ref())
    .await;
  match result {
    Ok(Some(person)) => {
      if verify_password(&user.sandi, &person.sandi) {
        match buat_token_jwt(person.id) {
          Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "status": true, "token": token })),
          Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"status": false, "message": "Gagal membuat token"})),
        }
      } else {
        HttpResponse::Unauthorized().json(serde_json::json!({"status": false, "message": "Sandi tidak valid"}))
      }
    }
    _ => HttpResponse::NotFound().json(serde_json::json!({"status": false, "message": "Email atau sandi salah"})),
  }
}

pub async fn update_users(id: web::Path<i64>, user: web::Json<Users>, koneksi: web::Data<SqlitePool>) -> impl Responder {
  let hashing: String = match hash_password(&user.sandi) {
    Ok(hashed) => hashed,
    Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({"status": false, "message": "Gagal menghash password"})),
  };
  let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("update users set nama = ?, email = ?, sandi = ? where id = ?")
    .bind(&user.nama)
    .bind(&user.email)
    .bind(hashing)
    .bind(id.into_inner())
    .execute(koneksi.get_ref())
    .await;
  match result {
    Ok(_) => HttpResponse::Ok().json(user.into_inner()),
    Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"status": false, "message": "Gagal memperbarui user"})),
  }
}

pub async fn hapus_users(id: web::Path<i64>, user: web::Json<Users>, koneksi: web::Data<SqlitePool>) -> impl Responder {
  let id_value: i64 = id.into_inner();
  let userid: i64 = user.id;
  if id_value != userid {
    return HttpResponse::BadRequest().json(serde_json::json!({"status": false, "message": "Tidak dapat menghapus orang lain"}));
  }
  let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("delete from users where id = ?")
    .bind(id_value)
    .execute(koneksi.get_ref())
    .await;
  match result {
    Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": true, "message": format!("User dengan id {} berhasil dihapus", id_value)})),
    Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"status": false, "message": "Gagal menghapus user"})),
  }
}