use futures::executor::block_on;
use planner::engine::db_wrapper::user::{delete_user, insert_user, login, User};
use planner::engine::db_wrapper::Connection;

#[test]
fn check_login() {
    let connection = Connection::new();
    let pool = &connection.unwrap().pool;
    let username = "pswdds";
    let password = "dasdasdasd";
    let user = User::new().username(username).password(password);

    let insert = block_on(insert_user(&pool, &user));
    assert_eq!(insert, true);

    login(&pool, username, password)
        .unwrap()
        .expect("Login returned none.");
    let delete = block_on(delete_user(&pool, &user));

    assert!(delete.is_none());
}
