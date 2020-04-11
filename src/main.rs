use std::env;
use dotenv;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use env_logger;

#[macro_use]
extern crate log;

async fn index(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
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
      .route("/", web::get().to(index))
      .route("/{name}", web::get().to(index))
    )
    .bind(target)?
    .run()
    .await
}
