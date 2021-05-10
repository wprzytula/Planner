fn main() {
    // let res = planner::interface::TUI::mainloop();
    // match res {
    //     Ok(()) => (),
    //     Err(_) => println!("Interface error occured."),
    // }
    planner::interface::GUI::gui_main(None);
}
