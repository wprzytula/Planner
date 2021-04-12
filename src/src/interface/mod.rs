// [TODO]: Interface of the Planner.

// [TODO: remove this reminder
/// Creating an engine for adding, removing, searching, displaying and
/// modifying events.

use sqlx::Error;
use crate::engine::db_wrapper::user::get_test_user;
use crate::engine::db_wrapper::Connection;
use crate::engine::db_wrapper::event::{Event, duration_from, Hours, ConversionError, Minutes};
use sqlx::postgres::types::PgInterval;
use std::io::Write;
use chrono::{DateTime, Utc, Date, NaiveDateTime, NaiveDate, NaiveTime, FixedOffset, TimeZone};
use chrono::offset::LocalResult::Single;

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

// impl From<std::num::ParseIntError> for InterfaceError {
//     fn from(_: std::num::ParseIntError) -> Self {
//         InterfaceError
//     }
// }

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
            "3" => provide_new_event_info(&connection)?,
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
    if events.is_empty() {
        println!("Nothing here, apparently!")
    }
    for event in events {
        println!("{:?}", event);
    }
    Ok(())
}

fn provide_search_conditions() -> Result<(), InterfaceError> {
    Ok(())
}

fn provide_new_event_info(connection: &Connection) -> Result<(), InterfaceError> {
    let stdin = std::io::stdin();

    println!("Okey, let's provide us with required information about the event.");
    println!("What's gonna be a title?");

    let mut title = String::new();
    if let Err(_) = stdin.read_line(&mut title) {
        return Err(InterfaceError);
    } else if title.is_empty() {
        println!("Title must not be empty!");
        return Err(InterfaceError)
    }

    println!("What's gonna be a date of the event? (Or a start date, if spans several days)");
    println!("(Use format: YYYY-MM-DD)");

    let mut date = String::new();
    let mut time = String::new();
    // let mut year = String::new();
    // let mut month = String::new();
    // let mut day = String::new();
    // let mut year_p : u32 = 0;
    // let mut month_p : u32 = 0;
    // let mut day_p : u32 = 0;
    //
    // for datum in [("Year", &mut year, &mut year_p),
    //     ("Month", &mut month, &mut month_p),
    //     ("Day", &mut day, &mut day_p)].iter_mut() {
    //
    //     print!("{}:\t", datum.0);
    //     std::io::stdout().flush()?;
    //     if let Err(_) = stdin.read_line(&mut datum.1) {
    //         return Err(InterfaceError);
    //     }
    //     datum.1.truncate(datum.1.trim_end().len());
    //     *datum.2 = match datum.1.trim().parse::<u32>() {
    //         Ok(x) => x,
    //         Err(_) => {
    //             println!("Invalid number: {}", datum.1);
    //             return Err(InterfaceError)
    //         }
    //     };
    // }
    if let Err(_) = stdin.read_line(&mut date) {
        return Err(InterfaceError);
    }

    let date_p = match NaiveDate::parse_from_str(&date.trim()[..], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid date format: {}", date);
            return Err(InterfaceError)
        }
    };

    println!("What's gonna be start time of the event?");
    println!("(Use format: HH:MM)");
    if let Err(_) = stdin.read_line(&mut time) {
        return Err(InterfaceError);
    }

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
        if let Err(_) = stdin.read_line(&mut datum.1) {
            return Err(InterfaceError);
        }
        datum.1.truncate(datum.1.trim_end().len());
        *datum.2 = match datum.1.trim().parse::<u32>() {
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
        if let Err(_) = crate::engine::add_event(&connection.pool, &get_test_user(), &new_event) {
            println!("Error occured while adding the event.");
            return Err(InterfaceError);
        } else {
            println!("Event successfully added to database.");
        }
    }

    Ok(())
}

fn choose_event_to_be_deleted() -> Result<(), InterfaceError> {
    Ok(())
}
