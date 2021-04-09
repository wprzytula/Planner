use sqlx::PgPool;
use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;

fn main() -> Result<(), sqlx::Error> {
    let pool = block_on(connect()).unwrap();
    let event = block_on(planner::event::get_event_by_id(&pool, 1)).unwrap();

    println!("Got event: {}", event.title);
    
    let event = block_on(
        planner::event::insert(
            &pool, "dupa", &(chrono::Utc::now() + chrono::Duration::days(69)),
            &sqlx::postgres::types::PgInterval { months: 21, days: 37, microseconds: 1488 },
            None
        )
    ).unwrap();

    println!("Added event: {:#?}", event);

    let delete_result = block_on(
        planner::event::delete_by_id(&pool, event.id)
    ).unwrap();

    println!("Deleted: {} rows", delete_result.rows_affected());

    let events = block_on(
        planner::event::get_all_events(&pool)
    ).unwrap();
    println!("Events currently in the db:");
    for event in events {
        println!("{:?}", event);
    }
    Ok(())
}

async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://51.38.126.54:8237/adam?user=adam&password=adam2137").await?;
    Ok(pool)
}
