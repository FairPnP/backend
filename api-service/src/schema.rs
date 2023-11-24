// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Int4,
        team_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    building_user_parking_spaces (id) {
        id -> Int4,
        building_user_id -> Int4,
        name -> Varchar,
        last_modified -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    building_users (id) {
        id -> Int4,
        building_id -> Int4,
        user_id -> Uuid,
        last_modified -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    buildings (id) {
        id -> Int4,
        name -> Varchar,
        place_id -> Varchar,
        latitude -> Numeric,
        longitude -> Numeric,
        last_modified -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Uuid,
        name -> Varchar,
        avatar_url -> Nullable<Varchar>,
        last_modified -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_ratings (id) {
        id -> Int4,
        user_id -> Uuid,
        rated_by_user_id -> Uuid,
        star_rating -> Int4,
        last_modified -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(building_user_parking_spaces -> building_users (building_user_id));
diesel::joinable!(building_users -> buildings (building_id));

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    building_user_parking_spaces,
    building_users,
    buildings,
    user_profiles,
    user_ratings,
);
