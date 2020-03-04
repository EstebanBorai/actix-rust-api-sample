use crate::server::{ApiError, connection as db_connection};
use crate::schema::rental;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{Numeric};
use diesel::pg::Pg;
use diesel::deserialize::{FromSql};
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "rental"]
pub struct RentalMessage {
	pub building_type: String,
	pub price: f32,
	pub year_of_construction: i16,
	pub address: String,
	pub bedrooms: i16,
	pub bathrooms: i16,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "rental"]
pub struct Rental {
	pub id: Uuid,
	pub building_type: String,
	pub price: Numeric,
	pub year_of_construction: i16,
	pub address: String,
	pub bedrooms: i16,
	pub bathrooms: i16,
	pub created_at: NaiveDateTime,
	pub updated_at: Option<NaiveDateTime>,
}

impl Rental {
	pub fn find_all() -> Result<Vec<Self>, ApiError> {
		let conn = db_connection()?;

		let rentals = rental::table
			.load::<Rental>(&conn)?;

		Ok(rentals)
	}

	pub fn find(id: Uuid) -> Result<Self, ApiError> {
		let conn = db_connection()?;

		let res = rental::table
			.filter(rental::id.eq(id))
			.first(&conn)?;

		Ok(res)
	}

	pub fn create(rental: RentalMessage) -> Result<Self, ApiError> {
		let conn = db_connection()?;

		let as_rental = Rental::from(rental);
		let inserted = diesel::insert_into(rental::table)
			.values(as_rental)
			.get_result(&conn)?;

		Ok(inserted)
	}
}

impl From<RentalMessage> for Rental {
	fn from(rental: RentalMessage) -> Self {
		Rental {
			id: Uuid::new_v4(),
			building_type: rental.building_type,
			price: rental.price.ToSql(),
			year_of_construction: rental.year_of_construction,
			address: rental.address,
			bedrooms: rental.bedrooms,
			bathrooms: rental.bathrooms,
			created_at: Utc::now().naive_utc(),
			updated_at: None,
		}
	}
}

impl Serialize for Rental {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let df: f32 = 1.0;

		let mut state = serializer.serialize_struct("Rental", 8)?;
		state.serialize_field("id", &self.id)?;
		state.serialize_field("building_type", &self.building_type)?;
		state.serialize_field("price", &df)?;
		state.serialize_field("year_of_contruction", &self.year_of_construction)?;
		state.serialize_field("address", &self.address)?;
		state.serialize_field("bedrooms", &self.bedrooms)?;
		state.serialize_field("bathrooms", &self.bathrooms)?;
		state.serialize_field("created_at", &self.created_at)?;
		state.serialize_field("updated_at", &self.updated_at)?;
		state.end()
	}
}
