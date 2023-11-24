use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = crate::schema::boards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Board {
    pub id: i32,
    pub team_id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::boards)]
pub struct NewBoard {
    pub team_id: Uuid,
    pub name: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::boards)]
pub struct UpdateBoard {
    pub name: Option<String>,
}
