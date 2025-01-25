use sqlx::{Error as KesalahanSQL, Pool, Sqlite, SqlitePool};

pub mod users;

pub async fn koneksi_ke_database() -> Result<SqlitePool, KesalahanSQL> {
  let database_url: &str = &dotenv::var("DATABASE_URL").expect("DATABASE_URL harus ada di .env");
  if !std::path::Path::new(database_url).exists() {
    std::fs::File::create(database_url).expect("Tidak dapat membuat database");
  }
  let koneksi: Pool<Sqlite> = SqlitePool::connect(&format!("sqlite:{}", database_url)).await?;
  sqlx::query(
    "CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      nama TEXT NOT NULL,
      email TEXT NOT NULL,
      sandi TEXT NOT NULL
    )"
  )
    .execute(&koneksi)
    .await?;
  Ok(koneksi)
}