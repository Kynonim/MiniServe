/**
 * Author: Riky Ripaldo
 * Created At: 2025/01/25
 * Tools: Rust, OtherLib, ChatGPT, DeepSeek
*/

use dotenv::dotenv;
use actix_web::{web, App, HttpServer, middleware::Logger};
use miniserve::core::koneksi_ke_database;
use miniserve::routes::konfigurasi;
use sqlx::{Pool, Sqlite};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let koneksi: Pool<Sqlite> = koneksi_ke_database().await.expect("Tidak dapat terhubung ke database");

    HttpServer::new(move || {
        App::new()
          .app_data(web::Data::new(koneksi.clone()))
          .wrap(Logger::default())
          .wrap(
            actix_cors::Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec!["Content-Type"])
          )
          .configure(konfigurasi)
    })
      .bind(("127.0.0.1", 2025))?
      .run()
      .await
}