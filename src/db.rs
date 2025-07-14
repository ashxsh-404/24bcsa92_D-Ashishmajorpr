use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn connect_to_db() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to database")
}
