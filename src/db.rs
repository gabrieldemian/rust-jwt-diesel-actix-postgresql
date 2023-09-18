use crate::error::Error;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2::{self};
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type DB = diesel::pg::Pg;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// We only want to have 1 instance of the connection throuhout
// the entire process. This is what lazy static does.
lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

/// Run the migrations located at `/migrations`.
pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

/// Connect to the database and run the migrations scripts.
pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("🔥 Failed to connect to the database");
    println!("✅ Connection to the database is successful!");
    run_db_migrations(&mut conn);
}

/// Connect to the database.
pub fn connection() -> Result<DbConnection, Error> {
    POOL.get()
        .map_err(|e| Error::new(500, format!("Failed getting db connection: {}", e)))
}
