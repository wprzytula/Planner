// [TODO]: User in DB.

pub(crate) fn get_test_user() -> User {
    User::new().username("Spongebob").password("Squarepants")
}

use crate::engine::Error;
use djangohashers::Algorithm::Argon2;
use djangohashers::{check_password_tolerant, make_password_with_algorithm};
use futures::executor::block_on;
use sqlx::PgPool;

#[derive(Debug)]
pub struct User {
    username: String,
    password: String,
}

impl User {
    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn new() -> User {
        User {
            username: String::from(""),
            password: String::from(""),
        }
    }

    pub fn username(mut self, username: &str) -> User {
        self.username = String::from(username);

        self
    }
    pub fn password(mut self, password: &str) -> User {
        self.password = hash(password);

        self
    }
}

pub async fn insert_user(pool: &PgPool, user: &User) -> bool {
    let query = sqlx::query_as!(
        User,
        "INSERT INTO users ( username, password )
         VALUES ( $1, $2 )
         RETURNING *",
        user.username,
        user.password
    )
    .fetch_one(pool)
    .await;
    println!("{:#?}", query);
    return query.is_ok();
}

pub async fn delete_user(pool: &PgPool, user: &User) -> Option<Error> {
    let query = sqlx::query!(
        "DELETE FROM users
        WHERE username = $1",
        user.username
    )
    .execute(pool)
    .await;

    match query {
        Ok(_) => None,
        Err(error) => Some(error),
    }
}

pub fn login(pool: &PgPool, username: &str, password: &str) -> Result<Option<User>, Error> {
    let user = block_on(get_password(pool, username))?;
    if user.is_none() {
        return Ok(None);
    }
    let user = user.unwrap();
    let hashed = &user.password[..];

    let result = check_hash(password, hashed);
    println!("{}", result);
    if result {
        return Ok(Some(user));
    }
    Ok(None)
}

async fn get_password(pool: &PgPool, username: &str) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub(self) fn hash(password: &str) -> String {
    make_password_with_algorithm(password, Argon2)
}

pub(self) fn check_hash(password: &str, hash: &str) -> bool {
    check_password_tolerant(password, hash)
}

#[cfg(test)]
mod tests {
    use crate::engine::db_wrapper::user::{check_hash, hash};
    use djangohashers::check_password;

    #[test]
    fn test_good_hash() {
        let psw = "I Like Eating Salt :)";
        assert!(check_hash(psw, &*hash(psw)));
    }

    #[test]
    fn test_bad_hash() {
        let wrg_psw = "I Don't Like Eating Salt :(";
        let psw = "I Like Eating Salt :)";
        assert!(!check_hash(wrg_psw, &*hash(psw)));
        assert!(!check_hash(psw, &*hash(wrg_psw)));
    }
}
