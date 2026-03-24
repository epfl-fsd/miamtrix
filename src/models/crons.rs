use diesel::prelude::*;
use crate::schema::crons;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cron {
    pub id: i32,
    pub room: String,
    pub cron_expression: String,
    pub command: String,
}

#[derive(Insertable)]
#[diesel(table_name = crons)]
pub struct NewCron<'a> {
    pub room: &'a str,
    pub cron_expression: &'a str,
    pub command: &'a str,
}
