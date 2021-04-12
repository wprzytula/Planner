use crate::engine::db_wrapper::user::User;
use crate::engine::{Error, EventId};
use sqlx::query;
use sqlx::PgPool;

pub async fn delete_event_from_schedule(pool: &PgPool, id: &EventId) -> Result<(), Error> {
    query!(
        "DELETE FROM schedule
        WHERE event = $1 ",
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn clear_user_schedule(pool: &PgPool, user: &User) -> Result<(), Error> {
    query!(
        "DELETE FROM schedule
        WHERE user = $1 ",
        user.get_username()
    )
    .execute(pool)
    .await?;

    Ok(())
}
