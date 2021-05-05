use planner::transport::*;
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

    let c = planner::engine::db_wrapper::Connection::new();
    let c = c.unwrap();

    let id : planner::transport::EventId = 2;

    let request = planner::transport::PlannerRequest {
        request_type: RequestType::DeleteEvent(id),
        author_username: String::from("testuser"),
    };

    let res = planner::transport::send_request(&c.pool, &request);
}
