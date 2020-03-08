use crate::server::ApiError;
use crate::user::{User, UserMessage};
use actix_web::{post, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use uuid::Uuid;

#[post("/register")]
async fn register(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
  let user = User::create(user.into_inner())?;

  Ok(HttpResponse::Ok().json(user))
}

#[post("sign-in")]
async fn sign_in(credentials: web::Json<UserMessage>, session: Session) -> Result<HttpResponse, ApiError> {
  let credentials = credentials.into_inner();

  let user = User::find_by_email(credentials.email)
    .map_err(|e| {
      match e.status_code {
        404 => ApiError::new(401, "Credentials are not valid!".to_string()),
        _ => e,
      }
    })?;

  let is_valid = user.verify_password(credentials.password.as_bytes())?;

  if is_valid == true {
    session.set("user_id", user.id)?;
  }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(register);
}
