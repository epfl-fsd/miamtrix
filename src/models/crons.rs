use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError}
};
use crate::schema::crons;
use crate::db::DbClient;
use crate::schema::crons::dsl::*;
use petname::petname;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cron {
    pub id: i32,
    pub name: String,
    pub room: String,
    pub cron_expression: String,
    pub command: String,
    pub job_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = crons)]
pub struct NewCron<'a> {
    pub name: &'a str,
    pub room: &'a str,
    pub cron_expression: &'a str,
    pub command: &'a str,
    pub job_id: &'a str
}

impl<'a> NewCron<'a> {
    pub fn create(
        target_room: &'a str,
        target_cron: &'a str,
        target_command: &'a str,
        target_job_id: &'a str,
    ) -> Cron {
        let mut conn = DbClient::get_connection();

        loop {
            let generated_name = Self::generate_name();

            let new_cron = NewCron {
                name: &generated_name,
                room: &target_room,
                cron_expression: &target_cron,
                command: &target_command,
                job_id: &target_job_id,
            };

            match diesel::insert_into(crons)
                .values(&new_cron)
                .returning(Cron::as_returning())
                .get_result(&mut conn)
            {
                Ok(cron) => {
                    return cron;
                }
                Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    continue;
                }
                Err(_) => {
                    println!("Failed to create new cron");
                    continue;
                }
            }
        }
    }
    fn generate_name() -> String {
        petname(2, " ").unwrap_or_else(|| "fallback name".to_string())
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
    pub fn delete_cron(target_room_id: &str, target_name: &str) -> bool {
        let mut conn = DbClient::get_connection();

        let deleted_cron = diesel::delete(crons.filter(room.eq(target_room_id).and(name.eq(target_name))))
            .execute(&mut conn);

        if !deleted_cron.is_err() {
            return true;
        }
        false
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
