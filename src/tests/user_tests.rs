use futures::executor::block_on;
use planner::engine::db_wrapper::user::{delete_user, insert_user, login, User};
use planner::engine::db_wrapper::{connect, disconnect};

#[test]
fn check_login() {
    let pool = block_on(connect()).unwrap();
    let user = User::new().username("testerek").password("testek");

    let insert = block_on(insert_user(&pool, &user));

    assert_eq!(insert, true);

    login(&pool, "testerek", "testek")
        .unwrap()
        .expect("Login returned none.");

    let delete = block_on(delete_user(&pool, &user));

    assert!(delete.is_none());

    block_on(disconnect(&pool));
}
