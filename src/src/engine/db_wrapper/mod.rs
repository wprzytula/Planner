use crate::engine::Error;
use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// [TODO] Move this to config file.
const DB_URI: &str = "postgres://adam:adam2137@51.38.126.54:8237/adam";

// [TODO]: Add session info.
pub struct Connection {
    pub pool: PgPool,
}

impl Connection {
    pub fn new() -> Result<Connection, Error> {
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

async fn connect() -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URI)
        .await?;
    Ok(pool)
}

async fn disconnect(pool: &PgPool) {
    pool.close().await;
}

pub fn begin_transaction(pool: &PgPool) -> Result<(), Error> {
    block_on(sqlx::query!("BEGIN").execute(pool))?;
    Ok(())
}

pub fn end_transaction(pool: &PgPool) -> Result<(), Error> {
    block_on(sqlx::query!("COMMIT").execute(pool))?;
    Ok(())
}

pub fn rollback_transaction(pool: &PgPool) -> Result<(), Error> {
    block_on(sqlx::query!("ROLLBACK").execute(pool))?;
    Ok(())
}

pub mod event;
pub mod schedule;
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
