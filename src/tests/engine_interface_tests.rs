use futures::executor::block_on;
use planner::engine;
use planner::engine::db_wrapper::event::Event;
use planner::engine::db_wrapper::{user, Connection};

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
