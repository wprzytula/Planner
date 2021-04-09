use futures::executor::block_on;
use planner::scheduler;

#[test]
fn add_and_remove() {
    let pool = block_on(scheduler::connect()).unwrap();

    let event = block_on(planner::event::insert(
        &pool,
        "dupa",
        &(chrono::Utc::now() + chrono::Duration::days(69)),
        &sqlx::postgres::types::PgInterval {
            months: 21,
            days: 37,
            microseconds: 1488,
        },
        None,
    ))
    .unwrap();

    block_on(planner::event::delete_by_id(&pool, event.id)).unwrap();

    block_on(pool.close());
}
