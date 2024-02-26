use crate::{
    api::validation::validate_req_data,
    error::ServiceError,
    services::postgres::{
        availability::{
            entities::{BuildingResult, SpaceResult},
            AvailabilityDb,
        },
        DbPool,
    },
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
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub lat_delta: BigDecimal,
    pub long_delta: BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct SearchAvailabilityResponse {
    pub buildings: Vec<BuildingResult>,
    pub spaces: Vec<SpaceResult>,
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

    let start_date = data.start_date.unwrap_or_else(|| {
        let now = chrono::Utc::now();
        NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap()
    });
    let end_date = data.end_date.unwrap_or_else(|| {
        // add a month to start date
        start_date + chrono::Duration::days(30)
    });

    let search_result = AvailabilityDb::search(
        &pool,
        start_date,
        end_date,
        data.latitude,
        data.longitude,
        data.lat_delta,
        data.long_delta,
    )
    .await?;

    let mut buildings: Vec<BuildingResult> = Vec::new();
    let mut spaces: Vec<SpaceResult> = Vec::new();
    let mut availabilities: Vec<PublicAvailability> = Vec::new();

    for result in search_result {
        let building = result.building;
        let space = result.space;
        let availability = result.availability;

        if !buildings.iter().any(|b| b.id == building.id) {
            buildings.push(building);
        }

        if !spaces.iter().any(|s| s.id == space.id) {
            spaces.push(SpaceResult {
                id: space.id,
                building_id: space.building_id,
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
