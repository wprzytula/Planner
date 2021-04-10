/* Module with interface for engine for the Planner. */

// [TODO]: Idea - create a new struct that will contain PgPool and logged user info.
//          The following functions would be then methods of this struct.

use crate::engine::db_wrapper::event::insert_event;
use futures::executor::block_on;
use sqlx::PgPool;

// [fixme]: This probably should not be public.
pub mod db_wrapper;

// [fixme]: Temporary definitions.
type Error = sqlx::Error;
// For now let us assume that we give only user's username (I am not sure if it's safe).
//type User = String;
type User = db_wrapper::user::User;
// This should be replaced with DTO or other temporary structure.
type NewEventInfo = db_wrapper::event::Event;
type EventModifyInfo = NewEventInfo;
type Event = db_wrapper::event::Event;
type EventId = i32;

// [TODO]: Some functions would love to use transactions in DB.
// [TODO]: Parameters and return types may change later!

pub fn add_event(pool: &PgPool, user: &User, event: &NewEventInfo) -> Result<EventId, Error> {
    // [TODO]: Should add both event and many-to-many record connecting the event to the user.
    begin_transaction(pool);
    let new_event = block_on(insert_event(pool, event));
    return match new_event {
        Ok(event) => {
            let scheduled = block_on(insert_scheduled_event(pool, event.id, user.get_username()));
            match scheduled {
                None => {
                    end_transaction(pool);
                    Ok(event.id)
                }
                Some(error) => {
                    rollback_transaction(pool).unwrap();
                    Err(error)
                }
            }
        }
        Err(error) => {
            rollback_transaction(pool).unwrap();
            Err(error)
        }
    };
    //Ok(42)
}

pub fn delete_event(pool: &PgPool, event_id: &EventId) -> Result<(), Error> {
    // [TODO]:
    Ok(())
}

pub fn modify_event(pool: &PgPool, event: &EventModifyInfo) -> Result<(), Error> {
    // [TODO]: Change it into more functions? Eg. modify duration, modify start time etc.
    Ok(())
}

pub fn get_all_user_events(pool: &PgPool, user: &User) -> Result<Vec<Event>, Error> {
    // [TODO]: test this baby
    block_on(async {
        sqlx::query_as!(
            Event,
            "SELECT E.*
            FROM schedule S
            LEFT JOIN events E
            ON S.event = E.id
            WHERE username = $1",
            user.get_username()
        )
        .fetch_all(pool)
        .await
    })
}

fn begin_transaction(pool: &PgPool) -> Option<Error> {
    block_on(async { sqlx::query!("BEGIN").execute(pool).await }).err()
}

fn end_transaction(pool: &PgPool) -> Option<Error> {
    block_on(async { sqlx::query!("COMMIT").execute(pool).await }).err()
}

fn rollback_transaction(pool: &PgPool) -> Option<Error> {
    block_on(async { sqlx::query!("ROLLBACK").execute(pool).await }).err()
}

async fn insert_scheduled_event(pool: &PgPool, event: i32, user: &str) -> Option<Error> {
    let result = sqlx::query!(
        "INSERT INTO schedule ( username, event )
         VALUES ( $1, $2 )",
        user,
        event
    )
    .execute(pool)
    .await;
    match result {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}
