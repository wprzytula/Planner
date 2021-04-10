/* Module with interface for engine for the Planner. */

// [TODO]: Idea - create a new struct that will contain PgPool and logged user info.
//          The following functions would be then methods of this struct.

use futures::executor::block_on;
use sqlx::{PgPool, Row};

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
    Ok(42)
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
