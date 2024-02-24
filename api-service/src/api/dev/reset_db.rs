use crate::{db::DbPool, error::ServiceError};
use actix_web::{post, web, HttpResponse};
use std::{collections::HashMap, fs};

#[post("reset-database/{table_name}")]
pub async fn reset_database(
    table_name: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let migration_base_path =
        std::env::var("MIGRATIONS_PATH").unwrap_or_else(|_| "./migrations".to_string());

    // map table name to migration path
    let table_map = HashMap::from([
        ("shared", "00000000000001_shared"),
        ("buildings", "2023-11-23-203951_create_buildings"),
        ("user_profiles", "2023-11-23-204915_create_user_profiles"),
        ("spaces", "2023-11-23-204931_create_spaces"),
        (
            "space_favorites",
            "2023-12-11-204427_create_space_favorites",
        ),
        ("availability", "2023-12-11-210720_create_availability"),
        ("reservations", "2023-12-11-210723_create_reservations"),
        ("stripe_accounts", "2024-01-05_create_stripe_accounts"),
        ("stripe_customers", "2024-01-08_create_stripe_customers"),
        (
            "reservation_chat_messages",
            "2024-01-13_create_reservation_chat_messages",
        ),
        ("space_images", "2024-01-28_space_images"),
        ("space_reviews", "2024-02-01_space_reviews"),
        ("space_summaries", "2024-02-01_space_summaries"),
        ("user_reviews", "2024-02-01_user_reviews"),
        ("user_summaries", "2024-02-01_user_summaries"),
        ("user_notif_tokens", "2024-02-24_user_notif_tokens"),
    ]);

    // get migration path
    let path = match table_map.get(table_name.as_str()) {
        Some(path) => path,
        None => return Err(ServiceError::BadRequest("Invalid table name".to_string())),
    };
    let path = format!("{}/{}", migration_base_path, path);

    // get db connection
    let mut script_results = String::new();

    // Run the down.sql script
    let down_script =
        fs::read_to_string(format!("{}/down.sql", path)).expect("Failed to read down script");
    let statements = down_script.split(";");

    for statement in statements {
        if statement.trim().is_empty() {
            continue;
        }

        let res = sqlx::query(statement).execute(pool.get_ref()).await;

        // append results
        let res_str = match res {
            Ok(res) => format!("{:?} down success: {:?}\n", path, res),
            Err(err) => format!("{:?} down failed: {:?}\n", path, err),
        };
        script_results.push_str(&res_str);
    }

    // Run the up.sql script
    let up_script =
        fs::read_to_string(format!("{}/up.sql", path)).expect("Failed to read up script");
    let mut statements = up_script.split(";");
    if table_name.as_str() == "shared" {
        statements = up_script.split("?");
    }

    for statement in statements {
        if statement.trim().is_empty() {
            continue;
        }

        let res = sqlx::query(statement).execute(pool.get_ref()).await;

        // append results
        let res_str = match res {
            Ok(res) => format!("{:?} up success: {:?}\n", path, res),
            Err(err) => format!("{:?} up failed: {:?}\n", path, err),
        };
        script_results.push_str(&res_str);
    }

    println!("{}", &script_results);

    Ok(HttpResponse::Ok().body(script_results))
}
