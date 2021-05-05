use futures::executor::block_on;
use planner::engine;
use planner::engine::db_wrapper::event::Event;
use planner::engine::db_wrapper::user::delete_user_from_database;
use planner::engine::db_wrapper::{user, Connection};
use planner::engine::{
    add_event, delete_event, delete_user, get_user_events_by_criteria, GetEventsCriteria,
};
use sqlx::postgres::types::PgInterval;

#[test]
fn find_event_by_date() {
    let pool = Connection::new();
    let pool = &pool.unwrap().pool;
    let user = user::User::new()
        .username("find_event_by_date")
        .password("Smile :)");
    let insert = block_on(user::insert_user(&pool, &user));
    assert!(insert);

    let from = chrono::Utc::now();
    let to = from + chrono::Duration::days(10);
    let criteria = engine::GetEventsCriteria::new().date_between(from, to);

    let events = engine::get_user_events_by_criteria(pool, &user, &criteria).unwrap();
    assert!(events.is_empty());

    let event = Event::new()
        .title("Just some event")
        .date(from + chrono::Duration::days(3));

    let added = add_event(pool, &user, &event).unwrap();
    let criteria = engine::GetEventsCriteria::new().date_between(from, to);

    let events = engine::get_user_events_by_criteria(pool, &user, &criteria).unwrap();
    assert_eq!(events.len(), 1);

    delete_event(pool, &added).unwrap();
    block_on(delete_user_from_database(pool, &user)).unwrap();
}

#[test]
fn find_event_by_duration() {
    let connection = Connection::new();
    let pool = &connection.unwrap().pool;
    let user = user::User::new()
        .username("duaration test")
        .password("Nananannana");
    let added = block_on(user::insert_user(pool, &user));

    assert!(added);

    let event = Event::new()
        .date(chrono::Utc::now() + chrono::Duration::days(12))
        .duration(PgInterval {
            months: 21,
            days: 3,
            microseconds: 7,
        });

    let min = PgInterval {
        months: 0,
        days: 0,
        microseconds: 0,
    };
    let max = PgInterval {
        months: 30,
        days: 0,
        microseconds: 0,
    };
    let bad_max = PgInterval {
        months: 20,
        days: 0,
        microseconds: 0,
    };
    let bad_min = PgInterval {
        months: 28,
        days: 0,
        microseconds: 0,
    };

    add_event(pool, &user, &event).unwrap();

    let criteria = engine::GetEventsCriteria::new().duration_between(min, max);
    let events = engine::get_user_events_by_criteria(pool, &user, &criteria);
    assert_eq!(events.unwrap().len(), 1);

    let max = PgInterval {
        months: 30,
        days: 0,
        microseconds: 0,
    };
    let criteria = engine::GetEventsCriteria::new().duration_between(bad_min, max);
    let events = engine::get_user_events_by_criteria(pool, &user, &criteria);
    assert_eq!(events.unwrap().len(), 0);

    let min = PgInterval {
        months: 0,
        days: 0,
        microseconds: 0,
    };

    let criteria = engine::GetEventsCriteria::new().duration_between(min, bad_max);
    let events = engine::get_user_events_by_criteria(pool, &user, &criteria);
    assert_eq!(events.unwrap().len(), 0);

    delete_user(pool, &user).unwrap();
}

#[test]
fn find_event_by_description() {
    let connection = Connection::new().unwrap();
    let pool = &connection.pool;

    let user = user::User::new()
        .username("description test searching")
        .password("hahahah i have enough...");
    assert!(block_on(user::insert_user(pool, &user)));

    let event_empty = Event::new().date(chrono::Utc::now() + chrono::Duration::days(3));
    let criteria_empty = engine::GetEventsCriteria::new();

    add_event(pool, &user, &event_empty).unwrap();
    let events = get_user_events_by_criteria(pool, &user, &criteria_empty);
    assert_eq!(events.unwrap().len(), 1);
    let event_desc = Event::new()
        .description(Option::from("Tesssting"))
        .date(chrono::Utc::now() + chrono::Duration::days(342));
    add_event(pool, &user, &event_desc).unwrap();

    let criteria = GetEventsCriteria::new().description_like("Tess");
    let events = get_user_events_by_criteria(pool, &user, &criteria);
    delete_user(pool, &user).unwrap();
    assert_eq!(events.unwrap().len(), 2);
}
