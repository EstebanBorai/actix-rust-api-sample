use std::env;
use dotenv;
use actix_web::{App, HttpRequest, HttpServer, Responder, get, HttpResponse};
use actix_web::middleware::Logger;
use env_logger;

#[macro_use]
extern crate log;

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

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

  let host: String = env::var("HOST").expect("Missing HOST env var");
  let port: String = env::var("PORT").expect("Missing PORT env var");
  let target = format!("{}:{}", host, port);

  info!("🚀  Server ready at {}", format!("http://{}",&target));

  HttpServer::new(||
      App::new()
      .wrap(Logger::default())
      .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
      .service(index)
    )
    .bind(target)?
    .run()
    .await
}
