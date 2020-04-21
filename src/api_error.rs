use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiError {
  pub status_code: u16,
  pub message: String,
}

impl ApiError {
  pub fn new(status_code: u16, message: String) -> ApiError {
    ApiError {
      status_code,
      message
    }
  }
}
