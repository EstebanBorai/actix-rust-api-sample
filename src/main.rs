#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate log;

use std::env;
use dotenv;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_redis::RedisSession;
use env_logger;

mod schema;
mod api_error;
mod db;
mod user;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  if cfg!(debug_assertions) {
    env::set_var("RUST_LOG", "warn,info,error,debug");
    env_logger::init();
    dotenv::dotenv().ok().expect("Unable to read .env file");
    warn!("⚠️   Running in development mode");
  } else {
    env::set_var("RUST_LOG", "warn,error");
  }

  db::init();

  let host: String = env::var("HOST").expect("Missing HOST env var");
  let port: String = env::var("PORT").expect("Missing PORT env var");
  let redis_host: String = env::var("REDIS_HOST").expect("Missing REDIS_HOST env var");
  let redis_port: String = env::var("REDIS_PORT").expect("Missing REDIS_PORT env var");
  let target = format!("{}:{}", host, port);

  info!("🚀  Server ready at {}", format!("http://{}",&target));

  HttpServer::new(move ||
      App::new()
      .wrap(Logger::default())
      .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
      .wrap(RedisSession::new(format!("{}:{}", redis_host, redis_port), &[0; 32]))
      .configure(user::init_routes)
      .configure(auth::init_routes)
    )
    .bind(target)?
    .run()
    .await
}
