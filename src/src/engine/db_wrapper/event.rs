use crate::engine::{Error, EventModifyRequest, GetEventsCriteria};
use chrono::{TimeZone, Utc};
use sqlx::postgres::types::PgInterval;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use std::arch::x86_64::_mm_test_all_ones;

const SECS_TO_DISTANT_YEAR: i64 = 10000000000;

pub struct ConversionError;

pub struct Hours(u32);

pub struct Minutes(u32);

impl Hours {
    pub fn new(hours: u32) -> Result<Self, ConversionError> {
        if hours < 24 {
            Ok(Self(hours))
        } else {
            Err(ConversionError)
        }
    }
}

impl Minutes {
    pub fn new(minutes: u32) -> Result<Self, ConversionError> {
        if minutes < 60 {
            Ok(Self(minutes))
        } else {
            Err(ConversionError)
        }
    }
}

pub fn duration_from(months: u32, days: u32, hours: Hours, minutes: Minutes) -> PgInterval {
    PgInterval {
        months: months as i32,
        days: days as i32,
        microseconds: ((hours.0 as i64 * 60) + minutes.0 as i64) * 60 * 1000_000
    }
}

#[derive(Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub date: chrono::DateTime<Utc>,
    pub duration: PgInterval,
    pub creation_date: chrono::DateTime<Utc>,
    pub description: Option<String>, // Can be null, so it must be an Option.
}

impl Event {
    pub fn new() -> Event {
        Event {
            id: 0,
            title: String::new(),
            date: chrono::offset::Utc::now(),
            duration: PgInterval {
                months: 0,
                days: 0,
                microseconds: 0,
            },
            creation_date: chrono::offset::Utc::now(),
            description: None,
        }
    }

    pub fn title(mut self, title: &str) -> Event {
        self.title = String::from(title);

        self
    }

    pub fn date(mut self, date: chrono::DateTime<Utc>) -> Event {
        self.date = date;

        self
    }

    pub fn duration(mut self, duration: PgInterval) -> Event {
        self.duration = duration;

        self
    }

    pub fn description(mut self, description: Option<&str>) -> Event {
        self.description = match description {
            None => None,
            Some(desc) => Some(String::from(desc))
        };

        self
    }
}

// [TODO] Try making this more generic (not only for postgres).
pub async fn get_event_by_id(pool: &PgPool, id: i32) -> Result<Event, Error> {
    let event = sqlx::query_as!(
        Event,
        "SELECT id, title, date, duration, creation_date, description \
             FROM events \
             WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(event)
}
/*
// [TODO] As above :3
// [TODO] Some fancy builder pattern?
pub async fn insert_event(
    pool: &PgPool,
    title: &str,
    date: &chrono::DateTime<Utc>,
    duration: &PgInterval,
    description: Option<String>,
) -> Result<Event, sqlx::Error> {
    let event = sqlx::query_as!(
        Event,
        "INSERT INTO events(title, date, duration, creation_date, description)
             VALUES($1, $2, $3, $4, $5)
             RETURNING *",
        title,
        date,
        duration,
        chrono::offset::Utc::now(),
        description
    )
    .fetch_one(pool)
    .await?;
    Ok(event)
}
*/
// Because of function signatures in engine/mod.rs written by Adam.
pub async fn insert_event(pool: &PgPool, event: &Event) -> Result<Event, Error> {
    let new_event = sqlx::query_as!(
        Event,
        "INSERT INTO events(title, date, duration, creation_date, description)
         VALUES ( $1, $2, $3, $4, $5 )
         RETURNING *",
        event.title,
        event.date,
        event.duration,
        chrono::offset::Utc::now(),
        event.description
    )
    .fetch_one(pool)
    .await?;
    Ok(new_event)
}
// [TODO] You know what :*
pub async fn delete_by_id(pool: &PgPool, id: &i32) -> Result<PgQueryResult, Error> {
    let query = sqlx::query!(
        "DELETE FROM events
             WHERE id = $1;",
        id
    )
    .execute(pool)
    .await?;
    Ok(query)
}

pub async fn get_all_events(pool: &PgPool) -> Result<Vec<Event>, Error> {
    let events = sqlx::query_as!(Event, "SELECT * FROM events")
        .fetch_all(pool) // -> Vec<Event>
        .await?;
    Ok(events)
}

pub async fn get_user_events_by_criteria(
    pool: &PgPool,
    user: &str,
    criteria: GetEventsCriteria,
) -> Result<Vec<Event>, Error> {
    let title = match criteria.title_like {
        Some(str) => str,
        None => String::new(),
    };
    let date = match criteria.date_between {
        Some(dates) => dates,
        None => (
            chrono::offset::Utc.timestamp(0, 0),
            chrono::offset::Utc.timestamp(SECS_TO_DISTANT_YEAR, 0),
        ),
    };
    let duration = match criteria.duration_between {
        Some(durations) => durations,
        None => (
            PgInterval {
                months: 0,
                days: 0,
                microseconds: 0,
            },
            PgInterval {
                months: 12000,
                days: 0,
                microseconds: 0,
            },
        ),
    };
    let creation_date = match criteria.creation_date_between {
        Some(dates) => dates,
        None => (
            chrono::offset::Utc.timestamp(0, 0),
            chrono::offset::Utc.timestamp(SECS_TO_DISTANT_YEAR, 0),
        ),
    };
    let description = match criteria.description_like {
        Some(str) => str,
        None => String::new(),
    };

    let events = sqlx::query_as!(
        Event,
        "SELECT id, title, date, duration, creation_date, description
            FROM schedule S
            LEFT JOIN events E
            ON S.event = E.id
            WHERE username = $1
                AND title LIKE '%' || $2 || '%'
                AND date BETWEEN $3 AND $4
                AND duration BETWEEN $5 AND $6
                AND creation_date BETWEEN $7 AND $8
                AND (description IS NULL
                     OR description LIKE '%' || $9 || '%')",
        user,
        title,
        date.0,
        date.1,
        duration.0,
        duration.1,
        creation_date.0,
        creation_date.1,
        description
    )
    .fetch_all(pool)
    .await?;
    Ok(events)
}

pub async fn modify_event(
    pool: &PgPool,
    request: EventModifyRequest,
) -> Result<PgQueryResult, Error> {
    let event = sqlx::query_as!(
        Event,
        "SELECT id, title, date, duration, creation_date, description \
             FROM events \
             WHERE id = $1",
        request.id
    )
    .fetch_one(pool)
    .await?;

    let new_event = set_update_info(request, event);

    let query = sqlx::query!(
        "UPDATE events \
            SET title = $2, date = $3, duration = $4, creation_date = $5, description = $6\
            WHERE id = $1",
        new_event.id,
        new_event.title,
        new_event.date,
        new_event.duration,
        new_event.creation_date,
        new_event.description
    )
    .execute(pool)
    .await?;
    Ok(query)
}

fn set_update_info(request: EventModifyRequest, event: Event) -> Event {
    Event {
        id: request.id,
        title: match request.title {
            Some(title) => title,
            None => event.title,
        },
        date: match request.date {
            Some(date) => date,
            None => event.date,
        },
        duration: match request.duration {
            Some(duration) => duration,
            None => event.duration,
        },
        creation_date: match request.creation_date {
            Some(creation_date) => creation_date,
            None => event.creation_date,
        },
        description: match request.description {
            Some(description) => description,
            None => event.description,
        },
    }
}
