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
