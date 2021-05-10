// [TODO]: Interface of the Planner.

use crate::engine::db_wrapper::event::{duration_from, Event, Hours, Minutes};
use crate::engine::db_wrapper::user::get_test_user;
use crate::engine::db_wrapper::Connection;
use crate::engine::{
    add_event, delete_event, get_all_user_events, get_user_event_by_id,
    get_user_events_by_criteria, GetEventsCriteria,
};
use chrono::offset::LocalResult::Single;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use sqlx::postgres::types::PgInterval;

use std::io;
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

fn datetime_to_utc(datetime: &NaiveDateTime) -> Result<DateTime<Utc>, InterfaceError> {
    let offset = FixedOffset::east(1 * 3600);
    match offset.from_local_datetime(datetime) {
        Single(dt) => Ok(Utc.from_utc_datetime(&dt.naive_utc())),
        _ => {
            println!("Date conversion error.");
            return Err(InterfaceError);
        }
    }
}

fn readline_stripped(buf: &mut String) -> io::Result<()> {
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
        println!("\nWhat are you willing to do? Enter corresponding number.");

        println!("(0 or EOF) Quit.");
        println!("(1) View all events.");
        println!("(2) Search for events satisfying conditions.");
        println!("(3) Add a new event.");
        println!("(4) Delete an event.");

        let choice = match get_string_stripped() {
            Ok(s) => {
                if s.is_empty() {
                    break;
                } else {
                    s
                }
            }
            Err(_) => return Err(InterfaceError),
        };

        match &choice[..] {
            "0" => break,
            "1" => {
                if let Err(_) = display_events(&connection) {
                    println!("Error occured while trying to display events.")
                }
            }
            "2" => {
                if let Err(_) = provide_search_conditions(&connection) {
                    println!("Error occured while trying to search for events.")
                }
            }
            "3" => {
                if let Err(_) = provide_new_event_info(&connection) {
                    println!("Error occured while trying to add an event.");
                }
            }
            "4" => {
                if let Err(_) = choose_event_to_be_deleted(&connection) {
                    println!("Error occured while trying to delete an event.");
                }
            }
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
    println!("Okey, let's provide us with some information about the sought events.");
    println!("Every condition is optional and can be omitted by leaving empty.");

    let mut criteria = GetEventsCriteria::new();

    println!("What's the title like? (provide any key part of it)");
    let title = get_string_stripped()?;
    if !title.is_empty() {
        criteria = criteria.title_like(&title);
    }

    println!("What's the description like? (provide any key part of it)");
    let description = get_string_stripped()?;
    if !description.is_empty() {
        criteria = criteria.description_like(&description);
    }

    println!("What's the oldest date and time to be queried? (format: YYYY-mm-dd HH:MM)");
    let datetime_old = get_string_stripped()?;

    let datetime_old = if !datetime_old.is_empty() {
        match NaiveDateTime::parse_from_str(&datetime_old[..], "%Y-%m-%d %H:%M") {
            Ok(dt) => datetime_to_utc(&dt)?,
            Err(_) => {
                println!("Invalid datetime format: {}", datetime_old);
                return Err(InterfaceError);
            }
        }
    } else {
        chrono::offset::Utc.timestamp(0, 0)
    };

    println!("What's the newest date and time to be queried? (format: YYYY-mm-dd HH:MM)");
    let datetime_new = get_string_stripped()?;

    let datetime_new = if !datetime_new.is_empty() {
        match NaiveDateTime::parse_from_str(&datetime_new[..], "%Y-%m-%d %H:%M") {
            Ok(dt) => datetime_to_utc(&dt)?,
            Err(_) => {
                println!("Invalid datetime format: {}", datetime_new);
                return Err(InterfaceError);
            }
        }
    } else {
        const SECS_TO_DISTANT_YEAR: i64 = 10000000000;
        chrono::offset::Utc.timestamp(SECS_TO_DISTANT_YEAR, 0)
    };

    criteria = criteria.date_between(datetime_old, datetime_new);

    match get_user_events_by_criteria(&connection.pool, &get_test_user(), &criteria) {
        Ok(events) => {
            println!("These are results of your query:");
            if events.is_empty() {
                println!("Nothing here, apparently!")
            }
            for event in events {
                println!("{}", event);
            }
        }
        Err(_) => return Err(InterfaceError),
    }

    Ok(())
}

fn provide_new_event_info(connection: &Connection) -> Result<(), InterfaceError> {
    println!("Okey, let's provide us with required information about the event.");
    println!("What's gonna be a title?");

    let title = get_string_stripped()?;
    if title.is_empty() {
        println!("Title must not be empty!");
        return Err(InterfaceError);
    }

    println!("What's gonna be a date of the event? (Or a start date, if spans several days)");
    println!("(Use format: YYYY-MM-DD)");

    let date = get_string_stripped()?;

    let date_p = match NaiveDate::parse_from_str(&date.trim()[..], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid date format: {}", date);
            return Err(InterfaceError);
        }
    };

    println!("What's gonna be start time of the event?");
    println!("(Use format: HH:MM)");
    let time = get_string_stripped()?;

    let time_p = match NaiveTime::parse_from_str(&time.trim()[..], "%H:%M") {
        Ok(time) => time,
        Err(_) => {
            println!("Invalid time format: {}", time);
            return Err(InterfaceError);
        }
    };

    let datetime = NaiveDateTime::new(date_p, time_p);

    let datetime_utc = datetime_to_utc(&datetime)?;

    println!("How long is the event going to last?");
    println!("(Use numbers only)");

    let mut months = String::new();
    let mut days = String::new();
    let mut hours = String::new();
    let mut minutes = String::new();
    let mut months_p: u32 = 0;
    let mut days_p: u32 = 0;
    let mut hours_p: u32 = 0;
    let mut minutes_p: u32 = 0;

    for datum in [
        ("Months", &mut months, &mut months_p),
        ("Days", &mut days, &mut days_p),
        ("Hours", &mut hours, &mut hours_p),
        ("Minutes", &mut minutes, &mut minutes_p),
    ]
    .iter_mut()
    {
        print!("{}:\t", datum.0);
        std::io::stdout().flush()?;
        readline_stripped(datum.1)?;
        *datum.2 = match datum.1.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid number: {}", datum.1);
                return Err(InterfaceError);
            }
        };
    }

    let hours_p = match Hours::new(hours_p) {
        Ok(x) => x,
        Err(_) => {
            println!("Too big (over 23) hours amount: {}", hours_p);
            return Err(InterfaceError);
        }
    };

    let minutes_p = match Minutes::new(minutes_p) {
        Ok(x) => x,
        Err(_) => {
            println!("Too big (over 59) minutes amount: {}", minutes_p);
            return Err(InterfaceError);
        }
    };

    let duration: PgInterval = duration_from(months_p, days_p, hours_p, minutes_p);

    println!("Okey, eventually: would you like to add some lengthy description?");
    println!("(Just strike return to leave it empty)");

    let mut description = String::new();
    readline_stripped(&mut description)?;
    let description = if description.is_empty() {
        None
    } else {
        Some(&description[..])
    };

    let new_event = Event::new()
        .title(&title)
        .date(datetime_utc)
        .duration(duration)
        .description(description);

    println!("Let's summarize your event briefly:");
    println!("Title: {}", title);
    println!("Start date & time: {} {}", date, time);
    println!(
        "Duration: {} months, {} days, {} hours, {} minutes",
        months, days, hours, minutes
    );
    println!(
        "Description: {}",
        if let Some(d) = description {
            d
        } else {
            "<no description>"
        }
    );

    let mut confirmation = String::new();
    while confirmation != "OK" && confirmation != "NO" {
        println!("Ready to confirm? If so, then enter 'OK' and press return. Else 'NO'");
        if let Err(_) = readline_stripped(&mut confirmation) {
            return Err(InterfaceError);
        }
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
        println!("Nothing here, apparently!");
        return Ok(());
    }
    for event in events {
        println!("Id: {}, title: {}", event.id, event.title);
    }

    println!("Enter id of the event you want to delete, EOF to cancel.");
    let choice = match get_string_stripped() {
        Ok(s) => {
            if s.is_empty() {
                return Ok(());
            } else {
                s
            }
        }
        Err(_) => return Err(InterfaceError),
    };

    let id = match choice.parse::<i32>() {
        Ok(c) => c,
        Err(_) => {
            println!("Invalid input - not a number.");
            return Err(InterfaceError);
        }
    };

    let to_delete = match get_user_event_by_id(&connection.pool, &get_test_user(), &id) {
        Ok(event) => event,
        Err(e) => {
            println!("Error querying event with id {}: {:?}", id, e);
            return Err(InterfaceError);
        }
    };
    println!("{}", to_delete);
    println!("Are you sure you want to delete the above event? ('OK' / any other input)");
    let decision = match get_string_stripped() {
        Ok(s) => match &s[..] {
            "OK" => true,
            _ => false,
        },
        Err(_) => return Err(InterfaceError),
    };
    if decision {
        match delete_event(&connection.pool, &id) {
            Ok(_) => println!("Successfully deleted the event."),
            Err(e) => {
                println!("{:?}", e);
                return Err(InterfaceError);
            }
        };
    } else {
        println!("Cancelled deleting.")
    }

    Ok(())
}
