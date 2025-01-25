pub mod verify;
pub mod middleware;

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::{core::users::{buat_users, get_users, hapus_users, login_users, update_users}, routes::middleware::validasi_auth};

pub fn konfigurasi(config: &mut web::ServiceConfig) {
  let auth_middleware = HttpAuthentication::bearer(validasi_auth);
  config.service(
    web::scope("/api")
      .service(
        web::scope("/public")
          .route("/signin", web::post().to(login_users))
          .route("/signup", web::post().to(buat_users))
      )
      .service(
        web::scope("/private")
          .wrap(auth_middleware)
          .route("/users", web::get().to(get_users))
          .route("/users/{id}", web::put().to(update_users))
          .route("/users/{id}", web::delete().to(hapus_users))
      )
  );
}