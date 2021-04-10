use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// [TODO] Move this somewhere higher?
const DB_URI: &str = "postgres://51.38.126.54:8237/adam?user=adam&password=adam2137";

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URI)
        .await?;
    Ok(pool)
}

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
