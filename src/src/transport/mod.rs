use crate::engine::NewEventRequest;
use crate::engine::Error;
use crate::engine;
use sqlx::PgPool;

pub enum RequestType {
    NewEvent(NewEventRequest)
}

pub struct PlannerRequest {
    pub request_type: RequestType,
    pub author_username: String,
    // [TODO]: Other options used for validation
}

// This function will be used by server after receiving a request,
// so we can pass PgPool here (because server will also do this).
pub fn handle_request(pool: &PgPool, request: &PlannerRequest) -> Result<(), Error> {
    match &request.request_type {
        RequestType::NewEvent(req) => {
            let user = engine::User::new()
                .username(&request.author_username).password("test");
            engine::add_event(pool, &user, &req);
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}