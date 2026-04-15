// @generated automatically by Diesel CLI.

diesel::table! {
    crons (id) {
        id -> Int4,
        room -> Varchar,
        name -> Varchar,
        cron_expression -> Varchar,
        command -> Varchar,
        job_id -> Varchar,
        hour -> Varchar,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        room -> Varchar,
        location -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(crons, locations,);
