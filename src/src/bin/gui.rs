use planner::transport::{send_request, ReturnType};
use planner::engine::db_wrapper::Connection;
use planner::transport::PlannerRequest;
use planner::transport::RequestType;

fn main() {
    let conn = Connection::new().unwrap();
    let ev = match send_request(&conn.pool, &PlannerRequest {
        request_type: RequestType::GetUserEventsByRelativeWeek(0),
        author_username: String::from("Spongebob")
    }).unwrap() {
        ReturnType::SingleEvent(ev) => vec![ev],
        ReturnType::ManyEvents(ev) => ev,
        _ => vec![]
    };
    println!("{:?}", ev);
    // planner::interface::GUI::gui_main(None);
}
