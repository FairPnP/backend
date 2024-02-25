use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{buildings::BuildingDb, spaces::SpaceDb, DbPool},
};
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
use google_maps::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSpaceRequest {
    #[validate(length(min = 1, max = 255))]
    pub place_id: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub max_vehicle_size: String,
    #[validate(length(min = 1))]
    pub coverage: String,
    pub height_clearance_cm: Option<i32>,
    pub access_restrictions: Option<String>,
    pub parking_instructions: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateSpaceResponse {
    pub space: PublicSpace,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_space(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    google: web::Data<GoogleMapsClient>,
    data: web::Json<CreateSpaceRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;

    // Fetch place details
    let place_details = google.place_details(data.place_id.clone()).execute().await;
    let place_details = match place_details {
        Ok(place_details) => {
            if let Some(result) = place_details.result {
                result
            } else {
                let error = "Place details not found";
                return Err(ServiceError::InternalError(error.to_string()));
            }
        }
        Err(e) => {
            let error = format!("Error fetching place details: {:?}", e);
            return Err(ServiceError::InternalError(error));
        }
    };

    // Extract address components
    let mut street_number = String::new();
    let mut street_name = String::new();
    let mut city = String::new();
    let mut state = String::new();
    let mut postal_code = String::new();
    let mut country = String::new();
    place_details
        .address_components
        .unwrap()
        .iter()
        .for_each(|c| {
            c.types.iter().for_each(|t| match t {
                PlaceType::StreetNumber => street_number = c.long_name.to_owned(),
                PlaceType::Route => street_name = c.long_name.to_owned(),
                PlaceType::Locality | PlaceType::Sublocality => city = c.long_name.to_owned(),
                PlaceType::AdministrativeAreaLevel1 => state = c.long_name.to_owned(),
                PlaceType::Country => country = c.long_name.to_owned(),
                PlaceType::PostalCode => postal_code = c.long_name.to_owned(),
                _ => {}
            });
        });

    let lat = place_details.geometry.unwrap().location.lat.to_string();
    let lat = BigDecimal::parse_bytes(lat.as_bytes(), 10).unwrap();
    let lng = place_details.geometry.unwrap().location.lng.to_string();
    let lng = BigDecimal::parse_bytes(lng.as_bytes(), 10).unwrap();

    // Insert building
    let building = BuildingDb::insert(
        &pool,
        place_details.name.unwrap(),
        place_details.place_id.unwrap(),
        lat,
        lng,
        street_number,
        street_name,
        city,
        state,
        postal_code,
        country,
    )
    .await?;

    // Insert space
    let space = SpaceDb::insert(
        &pool,
        user_id,
        building.id,
        data.name.to_owned(),
        data.description,
        data.max_vehicle_size,
        data.coverage,
        data.height_clearance_cm,
        data.access_restrictions,
        data.parking_instructions,
    )
    .await?;
    Ok(HttpResponse::Created().json(CreateSpaceResponse {
        space: space.into(),
    }))
}
