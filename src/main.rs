#[macro_use]
extern crate log;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod user;
mod rental;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env_logger::init();

	let host = env::var("HOST").expect("Host not set");
	let port = env::var("PORT").expect("Port not set");

	info!("Initializing");

	HttpServer::new(|| App::new()
		.configure(user::init_routes)
		.configure(rental::init_routes)
	).bind(format!("{}:{}", host, port))?
	.run()
	.await
}
