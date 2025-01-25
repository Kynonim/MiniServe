use rand::thread_rng;
use crate::models::JWTExpire;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use jsonwebtoken::errors::Error as KesalahanJWT;
use argon2::password_hash::{SaltString, PasswordHash, Error as KesalahanHash};
use argon2::{PasswordHasher, Argon2, PasswordVerifier};
use chrono::{DateTime, Duration, Utc};

pub fn buat_token_jwt(id: i32) -> Result<String, KesalahanJWT> {
  let rahasia: String = dotenv::var("JWT_KEY").expect("JWT_KEY harus ada di .env");
  let kadaluarsa: DateTime<Utc> = Utc::now() + Duration::seconds(dotenv::var("JWT_DURASI").unwrap().parse::<i64>().unwrap());

  let jwtexp: JWTExpire = JWTExpire {
    id: id.to_string(),
    time: kadaluarsa.timestamp() as i64,
  };

  encode(&Header::default(), &jwtexp, &EncodingKey::from_secret(rahasia.as_bytes()))
}

pub fn validasi_token_jwt(token: &str) -> Result<JWTExpire, KesalahanJWT> {
  let rahasia: String = dotenv::var("JWT_KEY").expect("JWT_KEY harus ada di .env");
  decode::<JWTExpire>(token, &DecodingKey::from_secret(rahasia.as_bytes()), &Validation::default()).map(|data| data.claims)
}

pub fn hash_password(sandi: &str) -> Result<String, KesalahanHash> {
  let salt: SaltString = SaltString::generate(&mut thread_rng());
  let argon2: Argon2<'_> = Argon2::default();
  Ok(argon2.hash_password(sandi.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(sandi: &str, hash: &str) -> bool {
  let hashed: PasswordHash<'_> = PasswordHash::new(hash).unwrap();
  Argon2::default().verify_password(sandi.as_bytes(), &hashed).is_ok()
}