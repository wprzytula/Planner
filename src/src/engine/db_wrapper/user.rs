// [TODO]: User in DB.

use crate::engine::Error;
use futures::executor::block_on;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

pub struct User {
    username: String,
    password: String,
}

// In my opinion the builder pattern will be perfect if we will add something
// to the User struct.
// I think a non-consuming builder is better than consuming one.
impl User {
    pub fn new(username: &str) -> User {
        User {
            username: String::from(username),
            password: "".to_string(),
        }
    }
    pub fn set_password(&mut self, password: &str) -> &mut User {
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

    return query.is_err();
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

pub(self) fn hash(password: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(password);

    let hash_string = format!("{:X}", hasher.finalize());

    hash_string
}

#[cfg(test)]
mod tests {
    use crate::engine::db_wrapper::user::hash;

    #[test]
    fn check_hash() {
        assert_eq!(
            hash("Test"),
            "532EAABD9574880DBF76B9B8CC00832C20A6EC113D682299550D7A6E0F345E25"
        )
    }

    #[test]
    fn check_uppercase() {
        assert_ne!(
            hash("Test"),
            "532eaabd9574880dbf76b9b8cc00832c20a6ec113d682299550d7a6e0f345e25"
        )
    }
}
