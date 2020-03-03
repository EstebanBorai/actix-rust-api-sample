use crate::user::User;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
async fn find_all() -> impl Responder {
	HttpResponse::Ok().json(
		vec![
			User { id: 1, email: "estebanborai@github.com".to_string() },
		]
	)
}

#[get("/users/{id}")]
async fn find() -> impl Responder {
	HttpResponse::Ok().json(
		User { id: 1, email: "estebanborai@github.com".to_string() }
	)
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
	HttpResponse::Created().json(user.into_inner())
}

#[put("/users/{id}")]
async fn update(user: web::Json<User>) -> impl Responder {
	HttpResponse::Ok().json(user.into_inner())
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
	HttpResponse::Ok().json(json!({
		"message": "Deleted"
	}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(find_all);
	cfg.service(find);
	cfg.service(create);
	cfg.service(update);
	cfg.service(delete);
}
