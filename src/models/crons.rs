use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError}
};
use crate::{db::DbPool, schema::crons};
use crate::schema::crons::dsl::*;
use petname::petname;

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = crons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cron {
    pub id: i32,
    pub name: String,
    pub room: String,
    pub cron_expression: String,
    pub hour: String,
    pub command: String,
    pub job_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = crons)]
pub struct NewCron<'a> {
    pub name: &'a str,
    pub room: &'a str,
    pub cron_expression: &'a str,
    pub hour: &'a str,
    pub command: &'a str,
    pub job_id: &'a str
}

impl<'a> NewCron<'a> {
    pub async fn create(
        pool: &DbPool,
        target_room: &'a str,
        target_cron: &'a str,
        target_command: &'a str,
        target_job_id: &'a str,
        target_hour: &'a str,
    ) -> Result<Cron, String> {

        let room_owned = target_room.to_string();
        let cron_owned = target_cron.to_string();
        let command_owned = target_command.to_string();
        let job_id_owned = target_job_id.to_string();
        let hour_owned = target_hour.to_string();

        let conn = pool.get().await.expect("Failed to get connection");
        conn.interact(move |conn| {
            loop {
                let generated_name = petname(2, " ")
                    .unwrap_or_else(|| "fallback-name".to_string());

                let new_cron = NewCron {
                    name: &generated_name,
                    room: &room_owned,
                    cron_expression: &cron_owned,
                    command: &command_owned,
                    job_id: &job_id_owned,
                    hour: &hour_owned
                };

                match diesel::insert_into(crons::table)
                    .values(&new_cron)
                    .returning(Cron::as_returning())
                    .get_result(conn)
                {
                    Ok(cron) => return Ok(cron),
                    Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                        continue;
                    }
                    Err(_) => return Err(format!("Failed to create new cron")),
                }
            }
        })
        .await
        .expect("Failed to create new cron")

    }
    fn generate_name() -> String {
        petname(2, " ").unwrap_or_else(|| "fallback name".to_string())
    }
}

impl Cron {
    pub async fn get_all(pool: &DbPool) -> Vec<Cron> {
        let conn = pool.get().await.expect("Failed to get connection");

        conn.interact(|conn| {
            crons::table.select(Cron::as_select())
            .load(conn)
        }).await
        .expect("interact failed")
        .expect("query failed")
    }
    pub async fn delete_cron(pool: &DbPool, target_room_id: &str, target_name: &str) -> bool {
        let conn = pool.get().await.expect("Failed to get connection");
        let room_owned = target_room_id.to_string();
        let name_owned = target_name.to_string();

        conn.interact(move |conn| {
            diesel::delete(
                crons::table.filter(
                    // .and() chaîne deux conditions : WHERE room = $1 AND name = $2
                    room.eq(&room_owned).and(name.eq(&name_owned))
                )
            )
            .execute(conn) // retourne Result<usize> = nombre de lignes supprimées
        })
        .await
        .expect("query failed")
        .map(|rows_deleted| rows_deleted > 0)
        .expect("Interact failed")
    }
    pub async fn update_cron(pool: &DbPool, cron: Cron, cron_id: i32) -> bool {
        let conn = pool.get().await.expect("Failed to get connection");

        conn.interact(move |conn| {
            diesel::update(crons::table.filter(id.eq(cron_id)))
                .set(&cron)   // AsChangeset génère le SET automatiquement
                .execute(conn)     // execute() retourne le nombre de lignes affectées
        })
        .await
        .expect("query failed")
        .map(|rows_affected| rows_affected > 0)
        .expect("Interact failed")

    }
    pub async fn get_by_room_id(pool: &DbPool, target_room_id: &str) -> Vec<Cron> {
        let room_owned = target_room_id.to_string();
        let conn = pool.get().await.expect("Failed to get connection");
        conn.interact(move |conn| {
            crons::table
                .filter(room.eq(&room_owned))
                .select(Cron::as_select())
                .load(conn)
        })
        .await
        .expect("query failed")
        .expect("Error")
    }
}
