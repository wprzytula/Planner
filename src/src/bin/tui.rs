fn main() {
    let res = planner::interface::TUI::mainloop();
    match res {
        Ok(()) => (),
        Err(_) => println!("Interface error occured."),
    }
}