use actix_web::web;
use crate::core::users::{buat_users, get_users, hapus_users, login_users, update_users};

pub mod verify;
pub mod middleware;

pub fn konfigurasi(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api")
     .service(
       web::scope("/public")
         .route("/signin", web::post().to(login_users)))
         .route("/signup", web::post().to(buat_users))
     .service(
       web::scope("/private")
         .route("/users", web::get().to(get_users))
         .route("/users", web::put().to(update_users))
         .route("/users", web::delete().to(hapus_users))
         .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(middleware::validasi_auth))
     )
  );
}
