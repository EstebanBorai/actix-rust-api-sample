#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_redis::RedisSession;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod schema;
mod server;
mod user;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env_logger::init();

	// Initialize Database
	server::init_database();

	let host = env::var("HOST").expect("Host not set");
	let port = env::var("PORT").expect("Port not set");
	let redis_host = env::var("REDIS_HOST").expect("Redis Host is missing");
	let redis_port = env::var("REDIS_PORT").expect("Redis Port is missing");

	info!("Initializing");

	let server = HttpServer::new(move || App::new()
		.wrap(RedisSession::new(format!("{}:{}", redis_host, redis_port), &[0; 32]))
		.configure(user::init_routes)
		.configure(auth::init_routes)
	);

	server.bind(format!("{}:{}", host, port))?
		.run()
		.await
}
