use crate::routes::verify::validasi_token_jwt;
use jsonwebtoken::errors::ErrorKind as KesalahanJWTK;
use actix_web::{dev::ServiceRequest, HttpMessage, Error as KesalahanActixWeb};
use actix_web::error::ErrorUnauthorized as KesalahanActixWebUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validasi_auth(req: ServiceRequest, res: BearerAuth) -> Result<ServiceRequest, (KesalahanActixWeb, ServiceRequest)> {
  let token: &str = res.token();
  match validasi_token_jwt(token) {
    Ok(data) => {
      req.extensions_mut().insert(data);
      Ok(req)
    }
    Err(e) => {
      let kind: &KesalahanJWTK = e.kind();
      let error = match *kind {
        KesalahanJWTK::ExpiredSignature => KesalahanActixWebUnauthorized("Token kadaluarsa".to_string()),
        _ => KesalahanActixWebUnauthorized("Token tidak valid".to_string()),
      };
      Err((error, req))
    }
  }
}