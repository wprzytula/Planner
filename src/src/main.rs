// Main imports.
use sqlx::{Connection, PgConnection, PgPool};
use futures::executor::block_on;
use sqlx::postgres::types::PgInterval;
use sqlx::postgres::PgPoolOptions;

// Other imports.
use chrono::Utc;

fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    let pool = block_on(connect()).unwrap();

    let event = block_on(get_event_by_id(&pool, 1)).unwrap();

    println!("Got event: {}", event.title);

    Ok(())
}

async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://51.38.126.54:8237/adam?user=adam&password=adam2137").await?;
    Ok(pool)
}

// Example on how to get objects from the database.

struct Event {
    id: i32,
    title: String,
    date: chrono::DateTime<Utc>,
    duration: PgInterval,
    creation_date: chrono::DateTime<Utc>,
    description: Option<String>     // Can be null, so it must be an Option.
}

// [TODO] Try making this more generic (not only for postgres).
async fn get_event_by_id(pool: &PgPool, id: i32) -> Result<Event, sqlx::Error> {
    let event = sqlx::query_as!(Event, "SELECT id, title, date, duration, creation_date, description FROM events WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(event)
}