use crate::engine;
use engine::Error;
use sqlx::PgPool;
use crate::engine::delete_event;

pub type NewEventRequest = engine::NewEventRequest;
pub type EventId = engine::EventId;

pub enum RequestType {
    NewEvent(NewEventRequest),
    DeleteEvent(EventId)
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
            delete_event(pool, id)?;
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}