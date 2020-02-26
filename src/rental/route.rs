use crate::rental::Rental;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/rentals")]
async fn find_all() -> impl Responder {
	HttpResponse::Ok().json(
		vec![
			Rental {
				id: 1,
				year_of_construction: 2012,
				building_type: "Detached House".to_string(),
				price: 750.000,
				address: "742 Evergreen Terrace".to_string(),
				bedrooms: 3,
				bathrooms: 2
			},
			Rental {
				id: 2,
				year_of_construction: 2020,
				building_type: "Loft".to_string(),
				price: 360.000,
				address: "1502 New Hope".to_string(),
				bedrooms: 4,
				bathrooms: 2
			}
		]
	)
}

#[get("/rentals/{id}")]
async fn find() -> impl Responder {
	HttpResponse::Ok().json(
		Rental {
			id: 1,
			year_of_construction: 2012,
			building_type: "Detached House".to_string(),
			price: 750.000,
			address: "742 Evergreen Terrace".to_string(),
			bedrooms: 3,
			bathrooms: 2
		}
	)
}

#[post("/rentals")]
async fn create(rental: web::Json<Rental>) -> impl Responder {
	HttpResponse::Created().json(rental.into_inner())
}

#[put("/rentals/{id}")]
async fn update(rental: web::Json<Rental>) -> impl Responder {
	HttpResponse::Ok().json(rental.into_inner())
}

#[delete("/rentals/{id}")]
async fn delete() -> impl Responder {
	HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(find_all);
	cfg.service(find);
	cfg.service(create);
	cfg.service(update);
	cfg.service(delete);
}
