use planner::transport::*;

fn main() {
    // let res = planner::interface::mainloop();
    // match res {
    //     Ok(()) => (),
    //     Err(_) => println!("Interface error occured."),
    // }

    let c = planner::engine::db_wrapper::Connection::new();
    let c = c.unwrap();

    let id: planner::transport::EventId = 2;

    let new_user = RegisterUserRequest::new().username("tomek").password("atomek");

    let request = planner::transport::PlannerRequest {
        request_type: RequestType::RegisterUser(new_user),
        author_username: String::from(""),
    };

    let _res = planner::transport::send_request(&c.pool, &request);
}
