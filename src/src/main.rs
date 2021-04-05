// Main imports.
use sqlx::{Connection, PgConnection, PgPool};
use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;

fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    let pool = block_on(connect()).unwrap();

    let event = block_on(planner::user::get_event_by_id(&pool, 1)).unwrap();

    println!("Got event: {}", event.title);

    Ok(())
}

async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://51.38.126.54:8237/adam?user=adam&password=adam2137").await?;
    Ok(pool)
}
