fn main() {
    let res = planner::interface::mainloop();
    match res {
        Ok(()) => (),
        Err(_) => println!("Interface error occured."),
    }
}