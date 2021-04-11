use futures::executor::block_on;
use planner::engine::db_wrapper::event::Event;
use planner::engine::db_wrapper::user::User;
use planner::engine::*;

fn main() {
    // [TODO]: Move these tests to test module.
    let pool = block_on(db_wrapper::connect()).unwrap();
    let event = block_on(db_wrapper::event::get_event_by_id(&pool, 1)).unwrap();

    println!("Got event: {}", event.title);

    /*let event = block_on(db_wrapper::event::insert_event(
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

        println!("Added event: {:#?}", event);

        let delete_result = block_on(db_wrapper::event::delete_by_id(&pool, event.id)).unwrap();

        println!("Deleted: {} rows", delete_result.rows_affected());
    */
    let _ev = Event::new().title("dupa").description("xxx");
    let events = block_on(db_wrapper::event::get_all_events(&pool)).unwrap();
    println!("Events currently in the db:");
    for event in events {
        println!("{:?}", event);
    }
    let user = User::new().username("tester").password("test");

    let testers_events = get_all_user_events(&pool, &user).unwrap();

    println!("All events of user {}:", user.get_username());
    for event in testers_events {
        println!("{:?}", event);
    }

    let modify_request = EventModifyRequest {
        id: 58,
        title: Some(String::from("Nowy tytuł")),
        date: None,
        duration: None,
        creation_date: None,
        description: None,
    };

    let _res = modify_event(&pool, modify_request);
    let modified_event = block_on(db_wrapper::event::get_event_by_id(&pool, 58)).unwrap();
    println!("Event after modification: {}", modified_event.title);

    let modify_request = EventModifyRequest {
        id: 58,
        title: Some(String::from("test_wszystkich")),
        date: None,
        duration: None,
        creation_date: None,
        description: None,
    };
    let _res = modify_event(&pool, modify_request);
    let modified_event = block_on(db_wrapper::event::get_event_by_id(&pool, 58)).unwrap();
    println!("Event after returning its title: {}", modified_event.title);
}
