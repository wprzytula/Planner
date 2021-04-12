use futures::executor::block_on;
use planner::engine;
use planner::engine::db_wrapper::event::Event;
use planner::engine::db_wrapper::user::delete_user;
use planner::engine::db_wrapper::{user, Connection};
use planner::engine::{add_event, delete_event};

#[test]
fn find_events_with_name_like_wszy() {
    let pool = Connection::new();
    let pool = &pool.unwrap().pool;
    let user = user::User::new().username("tester");
    let criteria = engine::GetEventsCriteria::new().title_like("wszy");

    let events = engine::get_user_events_by_criteria(pool, &user, criteria).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, 58);
}

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

    let events = engine::get_user_events_by_criteria(pool, &user, criteria).unwrap();
    assert!(events.is_empty());

    let event = Event::new()
        .title("Just some event")
        .date(from + chrono::Duration::days(3));

    let added = add_event(pool, &user, &event).unwrap();
    let criteria = engine::GetEventsCriteria::new().date_between(from, to);

    let events = engine::get_user_events_by_criteria(pool, &user, criteria).unwrap();
    assert_eq!(events.len(), 1);

    delete_event(pool, &added).unwrap();
    block_on(delete_user(pool, &user)).unwrap();
}
