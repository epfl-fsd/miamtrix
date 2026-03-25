use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection}
};
use std::sync::OnceLock;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub static DB_POOL: OnceLock<DbPool> = OnceLock::new();

pub struct DbClient;

impl DbClient {
    pub fn init(database_url: &str) {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create db pool");

        let _ = DB_POOL.set(pool);
    }

    pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        DB_POOL
            .get()
            .expect("DB is not initialised.")
            .get()
            .expect("Failed to get pool connection")
    }
}
