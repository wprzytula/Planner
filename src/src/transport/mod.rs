use crate::engine;
use engine::Error;
use sqlx::PgPool;

// HOW TO CREATE A NEW ENGINE FUNCTION:
// 1. Add a new public request type or choose an existing one.
// 2. Add a new enum type (obligatory).
// 3. Add a new branch in the handle_request function.
// 4. Call the engine function in this branch.
// 5. Add a new return type or choose and existing one.

pub type NewEventRequest = engine::NewEventRequest;
pub type EventId = engine::EventId;
pub type EventModifyRequest = engine::EventModifyRequest;
pub type GetEventsCriteria = engine::GetEventsCriteria;
pub type RegisterUserRequest = engine::User;

pub enum RequestType {
    NewEvent(NewEventRequest),
    DeleteEvent(EventId),
    ModifyEvent(EventModifyRequest),
    GetUserEventById(EventId),
    GetUserEventsByCriteria(GetEventsCriteria),
    RegisterUser(RegisterUserRequest),
    Login(String, String),
    // Not implemented:
    // delete_user (do we need this?)
    // get_all_user_event
}

pub type Event = engine::Event;

pub enum ReturnType {
    None,
    SingleEvent(Event),
    ManyEvents(Vec<Event>),
    WasSuccess(bool), // [TODO]: This is generally wrong, but we don't have a good error system.
    User(engine::User),
}

pub struct PlannerRequest {
    pub request_type: RequestType,
    pub author_username: String,
    // [TODO]: Other options used for validation.
}

// This function will be used by client, PgPool should be removed later.
pub fn send_request(pool: &PgPool, request: &PlannerRequest) -> Result<ReturnType, Error> {
    handle_request(pool, request)
}

// This function will be used by server after receiving a request,
// so we can pass PgPool here (because server will also do this).
pub fn handle_request(pool: &PgPool, request: &PlannerRequest) -> Result<ReturnType, Error> {
    // [TODO]: This user struct should have some additional validation info.
    let user = engine::User::new()
        .username(&request.author_username)
        .password("test");

    match &request.request_type {
        RequestType::NewEvent(req) => {
            engine::add_event(pool, &user, &req)?;
            Ok(ReturnType::None)
        }
        RequestType::DeleteEvent(id) => {
            engine::delete_event(pool, id)?;
            Ok(ReturnType::None)
        }
        RequestType::ModifyEvent(req) => {
            engine::modify_event(pool, req)?;
            Ok(ReturnType::None)
        }
        RequestType::GetUserEventById(id) => {
            let res = engine::get_user_event_by_id(pool, &user, id)?;
            Ok(ReturnType::SingleEvent(res))
        }
        RequestType::GetUserEventsByCriteria(criteria) => {
            let res = engine::get_user_events_by_criteria(pool, &user, criteria)?;
            Ok(ReturnType::ManyEvents(res))
        }
        RequestType::RegisterUser(new_user) => {
            let res = engine::register_user(pool, new_user);
            Ok(ReturnType::WasSuccess(res))
        }
        RequestType::Login(username, password) => {
            let res = engine::login(pool, username, password)?;
            match res {
                Some(us) => Ok(ReturnType::User(us)),
                None => Ok(ReturnType::None),
            }
        }
    }
}
