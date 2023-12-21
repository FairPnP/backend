use crate::{
    api::{
        buildings::public::PublicBuilding, spaces::public::PublicSpace,
        validation::validate_req_data,
    },
    db::{
        availability::{entities::AvailabilityResult, AvailabilityDb},
        DbPool,
    },
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicAvailability;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct SearchAvailabilityRequest {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub lat_delta: BigDecimal,
    pub long_delta: BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct SearchAvailabilityResponse {
    pub buildings: Vec<PublicBuilding>,
    pub spaces: Vec<PublicSpace>,
    pub availabilities: Vec<PublicAvailability>,
}

// ======================================================================
// Route

#[post("/search")]
pub async fn search_availability(
    pool: web::Data<DbPool>,
    data: web::Json<SearchAvailabilityRequest>,
) -> Result<HttpResponse, ServiceError> {
    let data = validate_req_data(data.into_inner())?;

    let search_result = AvailabilityDb::search(
        &pool,
        data.start_date,
        data.end_date,
        data.latitude,
        data.longitude,
        data.lat_delta,
        data.long_delta,
    )
    .await?;

    let mut buildings: Vec<PublicBuilding> = Vec::new();
    let mut spaces: Vec<PublicSpace> = Vec::new();
    let mut availabilities: Vec<PublicAvailability> = Vec::new();

    for result in search_result {
        let building = result.building;
        let space = result.space;
        let availability = result.availability;

        if !buildings.iter().any(|b| b.id == building.id) {
            buildings.push(PublicBuilding {
                id: building.id,
                name: building.name,
                place_id: building.place_id,
                latitude: building.latitude,
                longitude: building.longitude,
            });
        }

        if !spaces.iter().any(|s| s.id == space.id) {
            spaces.push(PublicSpace {
                id: space.id,
                building_id: space.building_id,
                name: space.name,
            });
        }

        availabilities.push(PublicAvailability {
            id: availability.id,
            space_id: availability.space_id,
            start_date: availability.start_date,
            end_date: availability.end_date,
            hourly_rate: availability.hourly_rate,
        });
    }

    Ok(HttpResponse::Ok().json(SearchAvailabilityResponse {
        buildings,
        spaces,
        availabilities,
    }))
}
