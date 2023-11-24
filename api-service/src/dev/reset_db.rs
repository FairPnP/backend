use crate::db::DbPool;
use actix_web::{post, web, HttpResponse, Responder};
use diesel::connection::SimpleConnection;
use std::fs;

#[post("reset-database")]
pub async fn reset_database(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let migration_base_path =
        std::env::var("MIGRATIONS_PATH").unwrap_or_else(|_| "./migrations".to_string());
    // Paths to your migrations
    let migration_paths = vec![format!(
        "{}/2023-11-17-202608_create_boards",
        migration_base_path
    )];

    let mut script_results = String::new();

    // Manually run the down and up scripts for each migration
    for path in migration_paths {
        // Run the down.sql script
        let down_script =
            fs::read_to_string(format!("{}/down.sql", path)).expect("Failed to read down script");
        let res = conn.batch_execute(&down_script);

        // append results
        let res_str = match res {
            Ok(res) => format!("{:?} down success: {:?}\n", path, res),
            Err(err) => format!("{:?} down failed: {:?}\n", path, err),
        };
        script_results.push_str(&res_str);

        // Run the up.sql script
        let up_script =
            fs::read_to_string(format!("{}/up.sql", path)).expect("Failed to read up script");
        let res = conn.batch_execute(&up_script);

        // append results
        let res_str = match res {
            Ok(res) => format!("{:?} up success: {:?}\n", path, res),
            Err(err) => format!("{:?} up failed: {:?}\n", path, err),
        };
        script_results.push_str(&res_str);

        println!("{}", &script_results);
    }

    HttpResponse::Ok().body(script_results)
}
