// [TODO]: Interface of the Planner.

// [TODO: remove this reminder
/// Creating an engine for adding, removing, searching, displaying and
/// modifying events.

use std::io;
use crate::engine::{get_all_user_events, add_event};
use crate::engine::db_wrapper::user::get_test_user;
use crate::engine::db_wrapper::Connection;
use crate::engine::db_wrapper::event::{Event, duration_from, Hours, Minutes};
use sqlx::postgres::types::PgInterval;
use chrono::{DateTime, Utc, NaiveDateTime, NaiveDate, NaiveTime, FixedOffset, TimeZone};
use chrono::offset::LocalResult::Single;
use std::io::Write;

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

fn readline_stripped(buf : &mut String) -> io::Result<()> {
    io::stdin().read_line(buf)?;
    buf.truncate(buf.trim_end().len());

    Ok(())
}

fn get_string_stripped() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf.truncate(buf.trim_end().len());

    Ok(buf)
}

pub fn mainloop() -> Result<(), InterfaceError> {
    welcome()?;

    let connection = Connection::new().expect("Failed to connect with Planner database! Exiting.");

    loop {
        println!("What are you willing to do? Enter corresponding number.");

        println!("(0 or EOF) Quit.");
        println!("(1) View all events.");
        println!("(2) Search for events satisfying conditions.");
        println!("(3) Add a new event.");
        println!("(4) Delete an event.");

        let choice = match get_string_stripped() {
            Ok(s) => if s.is_empty() {break} else {s},
            Err(_) => return Err(InterfaceError)
        };

        match &choice[..] {
            "0" => break,
            "1" => {
                if let Err(_) = display_events(&connection) {
                    println!("Error occured while trying to display events.")
                }
            },
            "2" => if let Err(_) = provide_search_conditions(&connection) {
                println!("Error occured while trying to search for events.")
            },
            "3" => if let Err(_) = provide_new_event_info(&connection) {
                println!("Error occured while trying to add an event.");
            },
            "4" => match choose_event_to_be_deleted(&connection) {
                Ok(_) => println!("Successfully deleted an event."),
                Err(_) => println!("Error occured while trying to delete an event.")
            },
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
    let events = get_all_user_events(&connection.pool, &get_test_user())?;
    if events.is_empty() {
        println!("Nothing here, apparently!")
    }
    for event in events {
        println!("{}", event);
    }
    Ok(())
}

fn provide_search_conditions(connection: &Connection) -> Result<(), InterfaceError> {
    Ok(())
}

fn provide_new_event_info(connection: &Connection) -> Result<(), InterfaceError> {
    let stdin = std::io::stdin();

    println!("Okey, let's provide us with required information about the event.");
    println!("What's gonna be a title?");

    let title = get_string_stripped()?;
    if title.is_empty() {
        println!("Title must not be empty!");
        return Err(InterfaceError)
    }

    println!("What's gonna be a date of the event? (Or a start date, if spans several days)");
    println!("(Use format: YYYY-MM-DD)");

    let date = get_string_stripped()?;

    let date_p = match NaiveDate::parse_from_str(&date.trim()[..], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid date format: {}", date);
            return Err(InterfaceError)
        }
    };

    println!("What's gonna be start time of the event?");
    println!("(Use format: HH:MM)");
    let time = get_string_stripped()?;

    let time_p = match NaiveTime::parse_from_str(&time.trim()[..], "%H:%M") {
        Ok(time) => time,
        Err(_) => {
            println!("Invalid time format: {}", time);
            return Err(InterfaceError)
        }
    };

    let offset = FixedOffset::east(1 * 3600);

    let datetime = NaiveDateTime::new(date_p, time_p);

    let datetime_utc: DateTime<Utc> = match offset.from_local_datetime(&datetime) {
        Single(dt) => Utc.from_utc_datetime(&dt.naive_utc()),
        _ => {
            println!("Date conversion error.");
            return Err(InterfaceError);
        }
    };

    println!("How long is the event going to last?");
    println!("(Use numbers only)");

    let mut months = String::new();
    let mut days = String::new();
    let mut hours = String::new();
    let mut minutes = String::new();
    let mut months_p : u32 = 0;
    let mut days_p : u32 = 0;
    let mut hours_p : u32 = 0;
    let mut minutes_p : u32 = 0;

    for datum in [("Months", &mut months, &mut months_p),
        ("Days", &mut days, &mut days_p),
        ("Hours", &mut hours, &mut hours_p),
        ("Minutes", &mut minutes, &mut minutes_p)].iter_mut() {

        print!("{}:\t", datum.0);
        std::io::stdout().flush()?;
        readline_stripped(datum.1)?;
        *datum.2 = match datum.1.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid number: {}", datum.1);
                return Err(InterfaceError)
            }
        };
    }

    let hours_p = match Hours::new(hours_p) {
        Ok(x) => x,
        Err(_) => {
            println!("Too big (over 23) hours amount: {}", hours_p);
            return Err(InterfaceError)
        }
    };
    let minutes_p = match Minutes::new(minutes_p) {
        Ok(x) => x,
        Err(_) => {
            println!("Too big (over 59) minutes amount: {}", minutes_p);
            return Err(InterfaceError)
        }
    };

    let duration : PgInterval = duration_from(months_p, days_p, hours_p, minutes_p);

    println!("Okey, eventually: would you like to add some lengthy description?");
    println!("(Just strike return to leave it empty)");

    let mut description = String::new();
    if let Err(_) = stdin.read_line(&mut description) {
        return Err(InterfaceError);
    }
    description.truncate(description.trim_end().len());

    let description = if description.is_empty() { None } else {Some(&description[..])};

    let new_event = Event::new().title(&title).date(datetime_utc).duration(duration).description(description);

    println!("Let's summarize your event briefly:");
    println!("Title: {}", title);
    println!("Start date & time: {} {}" , date, time);
    println!("Duration: {} months, {} days, {} hours, {} minutes", months, days, hours, minutes);
    println!("Description: {}", if let Some(d) = description {d} else {"<no description>"});

    let mut confirmation = String::new();
    while confirmation != "OK" && confirmation != "NO" {
        println!("Ready to confirm? If so, then enter 'OK' and press return. Else 'NO'");
        if let Err(_) = stdin.read_line(&mut confirmation) {
            return Err(InterfaceError);
        }
        confirmation.truncate(confirmation.trim_end().len());
    }
    if confirmation == "OK" {
        if let Err(_) = add_event(&connection.pool, &get_test_user(), &new_event) {
            return Err(InterfaceError);
        } else {
            println!("Successfully added an event.");
        }
    }

    Ok(())
}

fn choose_event_to_be_deleted(connection: &Connection) -> Result<(), InterfaceError> {
    let events = get_all_user_events(&connection.pool, &get_test_user())?;
    if events.is_empty() {
        println!("Nothing here, apparently!")
    }
    for event in events {
        println!("{}", event);
    }

    Ok(())
}
