use diesel::prelude::*;
use crate::schema::crons;
use crate::db::DbClient;
use crate::schema::crons::dsl::*;
use diesel:: prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cron {
    pub id: i32,
    pub room: String,
    pub cron_expression: String,
    pub command: String,
    pub job_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = crons)]
pub struct NewCron<'a> {
    pub room: &'a str,
    pub cron_expression: &'a str,
    pub command: &'a str,
    pub job_id: &'a str
}

impl<'a> NewCron<'a> {
    pub fn create(&self) -> Cron {
        let mut conn = DbClient::get_connection();

        diesel::insert_into(crons)
            .values(self)
            .returning(Cron::as_returning())
            .get_result(&mut conn)
            .expect("Failed create new cron")
    }
}

impl Cron {
    pub fn get_all() -> Vec<Cron> {
        let mut conn = DbClient::get_connection();

        crons
            .select(Cron::as_select())
            .load(&mut conn)
            .expect("Failed to load all crons")
    }

    pub fn get_by_room_id(target_room_id: &str) -> Vec<Cron> {
        use crate::schema::crons::dsl::*;
        let mut conn = DbClient::get_connection();

        crons
            .filter(room.eq(target_room_id))
            .select(Cron::as_select())
            .load(&mut conn)
            .expect("Failed to load cron by room")
    }
}
