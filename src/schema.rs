// @generated automatically by Diesel CLI.

diesel::table! {
    crons (id) {
        id -> Int4,
        room -> Varchar,
        cron_expression -> Varchar,
        command -> Varchar,
    }
}
