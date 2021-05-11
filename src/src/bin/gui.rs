use planner::transport::{send_request, ReturnType};
use planner::engine::db_wrapper::Connection;
use planner::transport::PlannerRequest;
use planner::transport::RequestType;

fn main() {
    planner::interface::GUI::gui_main(None);
}
