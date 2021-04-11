/* Module with interface for engine for the Planner. */

// [TODO]: Idea - create a new struct that will contain PgPool and logged user info.
//          The following functions would be then methods of this struct.

use crate::engine::db_wrapper::event::insert_event;
use chrono::Utc;
use futures::executor::block_on;
use sqlx::postgres::types::PgInterval;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

// [fixme]: This probably should not be public.
pub mod db_wrapper;

// [fixme]: Temporary definitions.
type Error = sqlx::Error;
// For now let us assume that we give only user's username (I am not sure if it's safe).
type User = db_wrapper::user::User;

// This should be replaced with DTO or other temporary structure.
type NewEventRequest = db_wrapper::event::Event;
type Event = db_wrapper::event::Event;
type EventId = i32;

pub struct EventModifyRequest {
    pub id: i32,
    pub title: Option<String>,
    pub date: Option<chrono::DateTime<Utc>>,
    pub duration: Option<PgInterval>,
    pub creation_date: Option<chrono::DateTime<Utc>>,
    pub description: Option<Option<String>>,
}

// [TODO]: Add references and lifetimes instead of taking over the ownership.
pub struct GetEventsCriteria {
    pub title_like: Option<String>,
    pub date_between: Option<(chrono::DateTime<Utc>, chrono::DateTime<Utc>)>,
    pub duration_between: Option<(PgInterval, PgInterval)>,
    pub creation_date_between: Option<(chrono::DateTime<Utc>, chrono::DateTime<Utc>)>,
    pub description_like: Option<String>,
}

impl GetEventsCriteria {
    pub fn new() -> GetEventsCriteria {
        GetEventsCriteria {
            title_like: None,
            date_between: None,
            duration_between: None,
            creation_date_between: None,
            description_like: None,
        }
    }

    pub fn title_like(mut self, title: &str) -> GetEventsCriteria {
        self.title_like = Some(String::from(title));
        self
    }

    pub fn date_between(
        mut self,
        from: chrono::DateTime<Utc>,
        to: chrono::DateTime<Utc>,
    ) -> GetEventsCriteria {
        self.date_between = Some((from, to));
        self
    }

    pub fn duration_between(mut self, from: PgInterval, to: PgInterval) -> GetEventsCriteria {
        self.duration_between = Some((from, to));
        self
    }

    pub fn creation_date_between(
        mut self,
        from: chrono::DateTime<Utc>,
        to: chrono::DateTime<Utc>,
    ) -> GetEventsCriteria {
        self.creation_date_between = Some((from, to));
        self
    }

    pub fn description_like(mut self, desc: &str) -> GetEventsCriteria {
        self.description_like = Some(String::from(desc));
        self
    }
}

// [TODO]: Some functions would love to use transactions in DB.
// [TODO]: Parameters and return types may change later!

// TODO: make those encapsulated into Connection

pub fn add_event(pool: &PgPool, user: &User, event: &NewEventRequest) -> Result<EventId, Error> {
    begin_transaction(pool)?;
    let new_event = block_on(insert_event(pool, event));
    return match new_event {
        Ok(event) => {
            let scheduled = block_on(insert_scheduled_event(pool, event.id, user.get_username()));
            match scheduled {
                Ok(()) => {
                    end_transaction(pool)?;
                    Ok(event.id)
                }
                Err(error) => {
                    rollback_transaction(pool)?;
                    Err(error)
                }
            }
        }
        Err(error) => {
            rollback_transaction(pool).unwrap();
            Err(error)
        }
    };
}

pub fn delete_event(pool: &PgPool, event_id: &EventId) -> Result<PgQueryResult, Error> {
    block_on(db_wrapper::event::delete_by_id(pool, event_id))
}

pub fn modify_event(pool: &PgPool, request: EventModifyRequest) -> Result<PgQueryResult, Error> {
    block_on(db_wrapper::event::modify_event(pool, request))
}

pub fn get_all_user_events(pool: &PgPool, user: &User) -> Result<Vec<Event>, Error> {
    // [TODO]: Move to db_wrapper
    block_on(
        sqlx::query_as!(
            Event,
            "SELECT E.*
            FROM schedule S
            LEFT JOIN events E
            ON S.event = E.id
            WHERE username = $1",
            user.get_username()
        )
        .fetch_all(pool),
    )
}

pub fn get_user_events_by_criteria(
    _pool: &PgPool,
    _user: &User,
    _criteria: &GetEventsCriteria,
) -> Result<Vec<Event>, Error> {
    Ok(Vec::new())
}

fn begin_transaction(pool: &PgPool) -> Result<(), Error> {
    block_on(sqlx::query!("BEGIN").execute(pool))?;
    Ok(())
}

fn end_transaction(pool: &PgPool) -> Result<(), Error> {
    block_on(sqlx::query!("COMMIT").execute(pool))?;
    Ok(())
}

fn rollback_transaction(pool: &PgPool) -> Result<(), Error> {
    block_on(sqlx::query!("ROLLBACK").execute(pool))?;
    Ok(())
}

// [TODO]: Move this to db_wrapper
async fn insert_scheduled_event(pool: &PgPool, event: i32, user: &str) -> Result<(), Error> {
    // [fixme] Unused result?
    let _result = sqlx::query!(
        "INSERT INTO schedule ( username, event )
         VALUES ( $1, $2 )",
        user,
        event
    )
    .execute(pool)
    .await?;
    Ok(())
}
