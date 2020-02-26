use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rental {
	pub id: i32,
	pub building_type: String,
	pub price: f64,
	pub year_of_construction: i32,
	pub address: String,
	pub bedrooms: i8,
	pub bathrooms: i8,
}
