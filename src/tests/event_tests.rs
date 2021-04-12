use futures::executor::block_on;
use planner::engine::db_wrapper::event::Event;
use planner::engine::db_wrapper::{event, Connection};

#[test]
fn add_and_remove() {
    let pool = Connection::new();
    let pool = &pool.unwrap().pool;
    let event = Event::new()
        .title("Test")
        .date(chrono::Utc::now() + chrono::Duration::days(69));
    let event = block_on(event::insert_event(&pool, &event)).unwrap();

    block_on(event::delete_by_id(&pool, &event.id)).unwrap();
}

#[test]
fn remove_nonexistent_event() {
    let pool = Connection::new();
    let pool = &pool.unwrap().pool;

    let event = Event::new().date(chrono::Utc::now() + chrono::Duration::days(23));
    let event = block_on(event::insert_event(&pool, &event)).expect("Inserting failed.");
    block_on(event::delete_by_id(&pool, &event.id)).unwrap();

    assert_eq!(
        block_on(event::delete_by_id(&pool, &event.id))
            .unwrap()
            .rows_affected(),
        0
    );
}
