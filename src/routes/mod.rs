use actix_web::web;
use crate::core::users::{get_users, buat_users};

pub mod auth;
pub mod verify;
pub mod middleware;

pub fn konfigurasi(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api")
     .service(
       web::scope("/public")
         .route("/signin", web::post().to(auth::login)))
         .route("/signup", web::post().to(buat_users))
     .service(
       web::scope("/private")
         .route("/users", web::get().to(get_users))
         .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(middleware::validasi_auth))
     )
  );
}
