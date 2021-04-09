use sqlx::postgres::types::PgInterval;
use sqlx::PgPool;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;

#[derive(Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub date: chrono::DateTime<Utc>,
    pub duration: PgInterval,
    pub creation_date: chrono::DateTime<Utc>,
    pub description: Option<String>     // Can be null, so it must be an Option.
}

// [TODO] Try making this more generic (not only for postgres).
pub async fn get_event_by_id(pool: &PgPool, id: i32) -> Result<Event, sqlx::Error> {
    let event = sqlx::query_as!(Event,
            "SELECT id, title, date, duration, creation_date, description \
             FROM events \
             WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(event)
}

// [TODO] As above :3
// [TODO] Some fancy builder pattern?
pub async fn insert(pool: &PgPool, title: &str, date: &chrono::DateTime<Utc>,
                    duration: &PgInterval, description: Option<String>) -> Result<Event, sqlx::Error> {
    let event = sqlx::query_as!(Event,
            "INSERT INTO events(title, date, duration, creation_date, description)
             VALUES($1, $2, $3, $4, $5)
             RETURNING *", title, date, duration,
                chrono::offset::Utc::now(), description)
        .fetch_one(pool)
        .await?;
    Ok(event)
}

// [TODO] You know what :*
pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    let query = sqlx::query!(
            "DELETE FROM events
             WHERE id = $1;", id)
        .execute(pool)
        .await?;
    Ok(query)
}

pub async fn get_all_events(pool: &PgPool) -> Result<Vec<Event>, sqlx::Error> {
    let events = sqlx::query_as!(Event,
            "SELECT * FROM events")
        .fetch_all(pool) // -> Vec<Event>
        .await?;
    Ok(events)
}