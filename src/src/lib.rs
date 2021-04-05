// [TODO]: Contents of this file probably should be moved to separate library.
pub mod user {
    use sqlx::postgres::types::PgInterval;
    use sqlx::PgPool;
    use chrono::Utc;
    // Example on how to get objects from the database.

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
        let event = sqlx::query_as!(Event, "SELECT id, title, date, duration, creation_date, description FROM events WHERE id = $1", id)
            .fetch_one(pool)
            .await?;
        Ok(event)
    }
}