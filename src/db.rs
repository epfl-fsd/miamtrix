use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection}
};
use std::sync::OnceLock;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub static DB_POOL: OnceLock<DbPool> = OnceLock::new();
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct DbClient;

impl DbClient {
    pub fn init(database_url: &str) {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create db pool");

        let _ = DB_POOL.set(pool);

        let mut conn = Self::get_connection();
        println!("Running database migrations ...");
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        println!("Migrations applied successfully");

    }

    pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        DB_POOL
            .get()
            .expect("DB is not initialised.")
            .get()
            .expect("Failed to get pool connection")
    }
}
