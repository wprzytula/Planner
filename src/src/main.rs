use planner::transport::RequestType;
use chrono::{DateTime, Utc};
use std::ops::Sub;
use sqlx::postgres::types::PgInterval;
use planner::engine::db_wrapper::event::{duration_from, Hours, Minutes, ConversionError};
use futures::executor::block_on;
use planner::engine::db_wrapper::Connection;
use sqlx::Error;

fn main() {
    // let res = planner::interface::mainloop();
    // match res {
    //     Ok(()) => (),
    //     Err(_) => println!("Interface error occured."),
    // }

    let pool = planner::engine::db_wrapper::Connection::new();
    let c = match pool {
        Ok(c) => {c}
        Err(_) => {panic!("conn")}
    };

    let hours = Hours::new(23);
    let hours = match hours {
        Ok(h) => {h}
        Err(err) => {panic!("hours error")}
    };

    let minutes = Minutes::new(12);
    let minutes = match minutes {
        Ok(m) => {m}
        Err(err) => {panic!("minutes error")}
    };

    let req = planner::engine::NewEventRequest::new()
        .title("test")
        .date(chrono::offset::Utc::now() + chrono::Duration::days(2))
        .duration(duration_from(1, 1, hours, minutes));

    let request = planner::transport::PlannerRequest {
        request_type: RequestType::NewEvent(req),
        author_username: String::from("testuser"),
    };

    let res = planner::transport::handle_request(&c.pool, &request);
}
