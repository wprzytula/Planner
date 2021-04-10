use futures::executor::block_on;
use planner::engine::db_wrapper::connect;
use planner::engine::db_wrapper::event;
use planner::engine::db_wrapper::event::Event;
use sqlx::postgres::types::PgInterval;

#[test]
fn add_and_remove() {
    let pool = block_on(connect()).unwrap();
    let event = Event::new()
        .title("Test")
        .date((chrono::Utc::now() + chrono::Duration::days(69)));
    let event = block_on(event::insert_event(&pool, &event)).unwrap();

    block_on(event::delete_by_id(&pool, event.id)).unwrap();

    block_on(pool.close());
}
