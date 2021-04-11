// [TODO]: Interface of the Planner.

// [TODO: remove this reminder
use crate::engine::db_wrapper::user::get_test_user;
use crate::engine::db_wrapper::Connection;
/// Creating an engine for adding, removing, searching, displaying and
/// modifying events.
use sqlx::Error;

pub struct InterfaceError;

impl From<sqlx::Error> for InterfaceError {
    fn from(_: sqlx::Error) -> Self {
        InterfaceError
    }
}

impl From<std::io::Error> for InterfaceError {
    fn from(_: std::io::Error) -> Self {
        InterfaceError
    }
}

pub fn mainloop() -> Result<(), InterfaceError> {
    welcome()?;
    let stdin = std::io::stdin();
    let mut line = String::new();

    let connection = Connection::new().expect("Failed to connect with Planner database! Exiting.");

    loop {
        line.truncate(0);

        println!("What are you willing to do? Enter corresponding number.");

        println!("(0 or EOF) Quit.");
        println!("(1) View all events.");
        println!("(2) Search for events satisfying conditions.");
        println!("(3) Add a new event.");
        println!("(4) Delete an event.");

        let nbytes = stdin.read_line(&mut line).expect("Stdin error.");
        if nbytes == 0 {
            break;
        }

        line = line.trim().parse().unwrap();

        match &line[..] {
            "0" => break,
            "1" => display_events(&connection)?,
            "2" => provide_search_conditions()?,
            "3" => provide_new_event_info()?,
            "4" => choose_event_to_be_deleted()?,
            _ => {
                println!("Unspecified input");
            }
        }
    }

    goodbye()?;
    Ok(())
}

fn welcome() -> Result<(), InterfaceError> {
    println!("Welcome to Planner!");
    println!("Prototype version 1.");
    Ok(())
}

fn goodbye() -> Result<(), InterfaceError> {
    println!("Goodbye!");
    Ok(())
}

fn display_events(connection: &Connection) -> Result<(), InterfaceError> {
    let events = crate::engine::get_all_user_events(&connection.pool, &get_test_user())?;
    for event in events {
        println!("{:?}", event);
    }
    Ok(())
}

fn provide_search_conditions() -> Result<(), InterfaceError> {
    Ok(())
}

fn provide_new_event_info() -> Result<(), InterfaceError> {
    Ok(())
}

fn choose_event_to_be_deleted() -> Result<(), InterfaceError> {
    Ok(())
}
