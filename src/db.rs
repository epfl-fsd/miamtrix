use diesel::{
    pg::PgConnection,
    Connection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");
pub type DbPool = Pool;


pub fn create_pool(database_url: &str) -> DbPool {
    let manager = Manager::new(database_url, Runtime::Tokio1);

    Pool::builder(manager)
        .max_size(10)
        .build()
        .unwrap_or_else(|e| {
            log::error!("Failed to create db pool : {}", e);
            panic!("Failed to create db pool");
        })
}

pub fn run_migrations(database_url: &str) {
    log::info!("Running database migrations ...");

    let mut conn = PgConnection::establish(database_url)
        .unwrap_or_else(|e| {
            log::error!("Failed to connect db for migrations : {}", e);
            panic!("Failed to connect db for migrations");
        });
    conn.run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|e| {
            log::error!("Failed to run migrations : {}", e);
            panic!("Failed to run migrations");
        });
    log::info!("Migrations applied successfully");
}
