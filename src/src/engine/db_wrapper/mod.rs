use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// [TODO] Move this somewhere higher?
const DB_URI: &str = "postgres://51.38.126.54:8237/adam?user=adam&password=adam2137";

// TODO: add session info
pub struct Connection {
    pub pool: PgPool,
}

impl Connection {
    pub fn new() -> Result<Connection, sqlx::Error> {
        match block_on(connect()) {
            Ok(pool) => Ok(Connection { pool }),
            Err(err) => Err(err),
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        block_on(disconnect(&self.pool));
    }
}

// TODO: remove pub, use RAII Connection instead
pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URI)
        .await?;
    Ok(pool)
}

// TODO: remove pub
pub async fn disconnect(pool: &PgPool) {
    pool.close().await;
}

pub mod event;
pub mod user;

#[cfg(test)]
mod unit_tests {
    use crate::engine::db_wrapper::{connect, disconnect};
    use futures::executor::block_on;

    #[test]
    fn connection_test() {
        let pool = block_on(connect()).unwrap();
        block_on(disconnect(&pool));
    }
}
