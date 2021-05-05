use crate::engine;
use engine::Error;
use sqlx::PgPool;

// HOW TO CREATE A NEW ENGINE FUNCTION:
// 1. Add a new public request type or choose an existing one.
// 2. Add a new enum type (obligatory).
// 3. Add a new branch in the handle_request function.

pub type NewEventRequest = engine::NewEventRequest;
pub type EventId = engine::EventId;
pub type EventModifyRequest = engine::EventModifyRequest;

pub enum RequestType {
    NewEvent(NewEventRequest),
    DeleteEvent(EventId),
    ModifyEvent(EventModifyRequest)
    // Not implemented:
    // delete_user
    // get_user_event_by_id
    // get_all_user_event
    // get_user_events_by_criteria
}

pub struct PlannerRequest {
    pub request_type: RequestType,
    pub author_username: String,
    // [TODO]: Other options used for validation
}

// This function will be used by client, PgPool should be removed later.
pub fn send_request(pool: &PgPool, request: &PlannerRequest) -> Result<(), Error> {
    handle_request(pool, request)
}

// This function will be used by server after receiving a request,
// so we can pass PgPool here (because server will also do this).
pub fn handle_request(pool: &PgPool, request: &PlannerRequest) -> Result<(), Error> {
    match &request.request_type {
        RequestType::NewEvent(req) => {
            let user = engine::User::new()
                .username(&request.author_username).password("test");
            engine::add_event(pool, &user, &req)?;
            Ok(())
        },
        RequestType::DeleteEvent(id) => {
            engine::delete_event(pool, id)?;
            Ok(())
        },
        RequestType::ModifyEvent(req) => {
            engine::modify_event(pool, req);
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}