// use crate::{
//     db::{get_db_connection, DbPool},
//     error::ServiceError,
// };
// use actix_web::{post, web, HttpResponse};
// use diesel::connection::SimpleConnection;
// use std::{collections::HashMap, fs};

// #[post("reset-database/{table_name}")]
// pub async fn reset_database(
//     table_name: web::Path<String>,
//     pool: web::Data<DbPool>,
// ) -> Result<HttpResponse, ServiceError> {
//     let migration_base_path =
//         std::env::var("MIGRATIONS_PATH").unwrap_or_else(|_| "./migrations".to_string());

//     // map table name to migration path
//     let table_map = HashMap::from([
//         ("buildings", "2023-11-23-203951_create_buildings"),
//         ("building_users", "2023-11-23-204859_create_building_users"),
//         ("user_profiles", "2023-11-23-204915_create_user_profiles"),
//         ("user_ratings", "2023-11-23-204922_create_user_ratings"),
//         (
//             "building_user_parking_spaces",
//             "2023-11-23-204931_create_building_user_parking_spaces",
//         ),
//     ]);

//     // get migration path
//     let path = match table_map.get(table_name.as_str()) {
//         Some(path) => path,
//         None => return Err(ServiceError::BadRequest("Invalid table name".to_string())),
//     };
//     let path = format!("{}/{}", migration_base_path, path);

//     // get db connection
//     let mut conn = get_db_connection(&pool)?;
//     let mut script_results = String::new();

//     // Run the down.sql script
//     let down_script =
//         fs::read_to_string(format!("{}/down.sql", path)).expect("Failed to read down script");
//     let res = conn.batch_execute(&down_script);

//     // append results
//     let res_str = match res {
//         Ok(res) => format!("{:?} down success: {:?}\n", path, res),
//         Err(err) => format!("{:?} down failed: {:?}\n", path, err),
//     };
//     script_results.push_str(&res_str);

//     // Run the up.sql script
//     let up_script =
//         fs::read_to_string(format!("{}/up.sql", path)).expect("Failed to read up script");
//     let res = conn.batch_execute(&up_script);

//     // append results
//     let res_str = match res {
//         Ok(res) => format!("{:?} up success: {:?}\n", path, res),
//         Err(err) => format!("{:?} up failed: {:?}\n", path, err),
//     };
//     script_results.push_str(&res_str);

//     println!("{}", &script_results);

//     Ok(HttpResponse::Ok().body(script_results))
// }
