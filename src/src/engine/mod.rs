/* Module with interface for engine for the Planner. */

use crate::engine::db_wrapper::event::insert_event;
use crate::engine::db_wrapper::schedule::{clear_user_schedule, delete_event_from_schedule};
use crate::engine::db_wrapper::user::{delete_user_from_database, insert_user};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Utc, Weekday};
use futures::executor::block_on;
use sqlx::postgres::types::PgInterval;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

// [fixme]: This probably should not be public.
pub mod db_wrapper;

pub type Error = sqlx::Error;
// For now let us assume that we give only user's username (I am not sure if it's safe).
pub type User = db_wrapper::user::User;

// This should be replaced with DTO or other temporary structure.
pub type NewEventRequest = db_wrapper::event::Event;
pub type Event = db_wrapper::event::Event;
pub type EventId = i32;

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

pub fn add_event(pool: &PgPool, user: &User, event: &NewEventRequest) -> Result<EventId, Error> {
    db_wrapper::begin_transaction(pool)?;
    let new_event = block_on(insert_event(pool, event));
    match new_event {
        Ok(event) => {
            if let Err(error) = block_on(db_wrapper::event::insert_scheduled_event(
                pool,
                event.id,
                user.get_username(),
            )) {
                db_wrapper::rollback_transaction(pool)?;
                Err(error)
            } else {
                db_wrapper::end_transaction(pool)?;
                Ok(event.id)
            }
        }
        Err(error) => {
            db_wrapper::rollback_transaction(pool).unwrap();
            Err(error)
        }
    }
}

pub fn delete_event(pool: &PgPool, event_id: &EventId) -> Result<PgQueryResult, Error> {
    db_wrapper::begin_transaction(pool).unwrap();

    return match block_on(delete_event_from_schedule(pool, event_id)) {
        Ok(_) => match block_on(db_wrapper::event::delete_by_id(pool, event_id)) {
            Ok(val) => {
                db_wrapper::end_transaction(pool).unwrap();
                Ok(val)
            }
            Err(error) => {
                db_wrapper::rollback_transaction(pool).unwrap();
                Err(error)
            }
        },
        Err(error) => {
            db_wrapper::rollback_transaction(pool).unwrap();
            Err(error)
        }
    };
}

pub fn modify_event(pool: &PgPool, request: &EventModifyRequest) -> Result<PgQueryResult, Error> {
    block_on(db_wrapper::event::modify_event(pool, request))
}

pub fn register_user(pool: &PgPool, user: &User) -> bool {
    block_on(insert_user(pool, user))
}

pub fn login(pool: &PgPool, username: &str, password: &str) -> Result<Option<User>, Error> {
    db_wrapper::user::login(pool, username, password)
}

pub fn delete_user(pool: &PgPool, user: &User) -> Result<(), Error> {
    db_wrapper::begin_transaction(pool).unwrap();

    return match block_on(clear_user_schedule(pool, user)) {
        Ok(_) => match block_on(delete_user_from_database(pool, user)) {
            Ok(_) => {
                db_wrapper::end_transaction(pool).unwrap();
                Ok(())
            }
            Err(error) => {
                db_wrapper::rollback_transaction(pool).unwrap();
                Err(error)
            }
        },
        Err(error) => {
            db_wrapper::rollback_transaction(pool).unwrap();
            Err(error)
        }
    };
}

// TODO: Check if queried event belongs to given user - crucial for security & privacy reasons
pub fn get_user_event_by_id(
    pool: &PgPool,
    _user: &User,
    event_id: &EventId,
) -> Result<Event, Error> {
    block_on(db_wrapper::event::get_event_by_id(pool, *event_id))
}

pub fn get_all_user_events(pool: &PgPool, user: &User) -> Result<Vec<Event>, Error> {
    block_on(db_wrapper::event::get_all_user_events(
        pool,
        user.get_username(),
    ))
}

pub fn get_user_events_by_criteria(
    pool: &PgPool,
    user: &User,
    criteria: &GetEventsCriteria,
) -> Result<Vec<Event>, Error> {
    let events = block_on(db_wrapper::event::get_user_events_by_criteria(
        pool,
        user.get_username(),
        criteria,
    ))?;
    Ok(events)
}

// [TODO]: Picking UTC may not be the best idea, but we don't have time.
pub fn get_desired_week(diff: i64) -> (DateTime<Utc>, DateTime<Utc>) {
    let future_date = chrono::offset::Utc::now() + Duration::weeks(diff);
    let current_year = future_date.year();
    let week = future_date.iso_week().week();

    let mon = NaiveDate::from_isoywd(current_year, week, Weekday::Mon)
        .and_time(NaiveTime::from_hms(0, 0, 0));
    let sun = NaiveDate::from_isoywd(current_year, week + 1, Weekday::Mon)
        .and_time(NaiveTime::from_hms(0, 0, 0));
    (
        Utc.from_local_datetime(&mon).unwrap(),
        Utc.from_local_datetime(&sun).unwrap(),
    )
}
