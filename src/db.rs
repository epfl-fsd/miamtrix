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
        .expect("Failed to create async db pool")
}

pub fn run_migrations(database_url: &str) {
    println!("Running database migrations ...");

    let mut conn = PgConnection::establish(database_url)
        .expect("Failed to connect for migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    println!("Migrations applied successfully");
}
