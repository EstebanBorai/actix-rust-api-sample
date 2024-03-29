use crate::api_error::ApiError;
use crate::db;
use crate::schema::user;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2::Config;
use rand::Rng;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
  pub id: Uuid,
  pub email: String,
  #[serde(skip_serializing)]
  pub password: String,
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserMessage {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
  pub email: Option<String>,
  pub sort_by: Option<String>,
  #[serde(rename = "created_at[gte]")]
  pub created_at_gte: Option<NaiveDateTime>,
  #[serde(rename = "created_at[lte]")]
  pub created_at_lte: Option<NaiveDateTime>,
  #[serde(rename = "updated_at[gte]")]
  pub updated_at_gte: Option<NaiveDateTime>,
  #[serde(rename = "updated_at[lte]")]
  pub updated_at_lte: Option<NaiveDateTime>,
}

impl From<UserMessage> for User {
  fn from(user: UserMessage) -> Self {
    User {
      id: Uuid::new_v4(),
      email: user.email,
      password: user.password,
      created_at: Utc::now().naive_utc(),
      updated_at: None,
    }
  }
}

impl User {
  pub fn find_all(params: Params) -> Result<Vec<Self>, ApiError> {
    let conn = db::connection()?;

    let mut query = user::table.into_boxed();

    if let Some(email) = params.email {
      query = query.filter(user::email.like(email))
    }

    if let Some(created_at_gte) = params.created_at_gte {
      query = query.filter(user::created_at.ge(created_at_gte));
    }

    if let Some(created_at_lte) = params.created_at_lte {
      query = query.filter(user::created_at.le(created_at_lte));
    }

    if let Some(updated_at_gte) = params.updated_at_gte {
      query = query.filter(user::updated_at.ge(updated_at_gte));
    }

    if let Some(updated_at_lte) = params.updated_at_lte {
      query = query.filter(user::updated_at.le(updated_at_lte));
    }

    if let Some(sort_by) = params.sort_by {
      query = match sort_by.as_ref() {
        "id" => query.order(user::id.asc()),
        "id.asc" => query.order(user::id.asc()),
        "id.desc" => query.order(user::id.desc()),
        "email" => query.order(user::email.asc()),
        "email.asc" => query.order(user::email.asc()),
        "email.desc" => query.order(user::email.desc()),
        "created_at" => query.order(user::created_at.asc()),
        "created_at.asc" => query.order(user::created_at.asc()),
        "created_at.desc" => query.order(user::created_at.desc()),
        "updated_at" => query.order(user::updated_at.asc()),
        "updated_at.asc" => query.order(user::updated_at.asc()),
        "updated_at.desc" => query.order(user::updated_at.desc()),
        _ => query,
      };
    }

    let users = query.load::<User>(&conn)?;

    Ok(users)
  }

  pub fn find(id: Uuid) -> Result<Self, ApiError> {
    let conn = db::connection()?;

    let user = user::table
      .filter(user::id.eq(id))
      .first(&conn)?;

    Ok(user)
  }

  pub fn find_by_email(email: String) -> Result<Self, ApiError> {
    let conn = db::connection()?;

    let user = user::table
      .filter(user::email.eq(email))
      .first(&conn)?;

    Ok(user)
  }

  pub fn create(user: UserMessage) -> Result<Self, ApiError> {
    let conn = db::connection()?;

    let mut user = User::from(user);
    user.hash_password()?;

    let user = diesel::insert_into(user::table)
      .values(user)
      .get_result(&conn)?;

    Ok(user)
  }

  pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
    let conn = db::connection()?;

    let user = diesel::update(user::table)
      .filter(user::id.eq(id))
      .set(user)
      .get_result(&conn)?;

    Ok(user)
  }

  pub fn delete(id: Uuid) -> Result<usize, ApiError> {
    let conn = db::connection()?;

    let res = diesel::delete(
      user::table
        .filter(user::id.eq(id))
    ).execute(&conn)?;

    Ok(res)
  }

  pub fn hash_password(&mut self) -> Result<(), ApiError> {
    let config = Config::default();
    let salt: [u8; 32] = rand::thread_rng().gen();

    self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
      .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

    Ok(())
  }

  pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
    argon2::verify_encoded(&self.password, password)
      .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
  }
}
