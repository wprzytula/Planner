// [TODO]: User in DB.

pub(crate) fn get_test_user() -> User {
    User::new().username("Spongebob").password("Squarepants")
}

use crate::engine::Error;
use djangohashers::Algorithm::Argon2;
use djangohashers::{check_password, make_password_with_algorithm, HasherError};
use futures::executor::block_on;
use sqlx::PgPool;

#[derive(Debug)]
pub struct User {
    username: String,
    password: String,
}

// In my opinion the builder pattern will be perfect if we will add something
// to the User struct.

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
    //println!("{:#?}", query);
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
    let hashed = hash(password);

    let result = block_on(authenticate(pool, username, &hashed[..]));

    result
}

async fn authenticate(
    pool: &PgPool,
    username: &str,
    hashed_password: &str,
) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE username = $1 AND
        password = $2",
        username,
        hashed_password
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}
/*
pub(self) fn hash(password: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(password);

    let hash_string = format!("{:X}", hasher.finalize());

    hash_string
}
*/
pub(self) fn hash(password: &str) -> String {
    let hash = make_password_with_algorithm(password, Argon2);
    hash
}

pub(self) fn check_hash(password: &str, hash: &str) -> Result<bool, HasherError> {
    check_password(password, hash)
}

#[cfg(test)]
mod tests {
    use crate::engine::db_wrapper::user::{check_hash, hash};
    use djangohashers::check_password;

    #[test]
    fn test_good_hash() {
        let psw = "I Like Eating Salt :)";
        assert!(check_hash(psw, &*hash(psw)).unwrap());
    }

    #[test]
    fn check_uppercase() {
        let wrg_psw = "I Don't Like Eating Salt :(";
        let psw = "I Like Eating Salt :)";
        assert!(!check_hash(wrg_psw, &*hash(psw)).unwrap());
        assert!(!check_hash(psw, &*hash(wrg_psw)).unwrap());
    }
}
