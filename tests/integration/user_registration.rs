/* User service integration test */

use crate::get_test_db_pool;

// use gandalf::domain::services::UserService;
use gandalf::adapters::repositories::{PgUserRepository, UserRepository};
use gandalf::domain::models::User;

#[tokio::test]
async fn check_email_exists() {
    let pool = get_test_db_pool().await;
    let user_repo = PgUserRepository::new(pool.clone());

    let email_exists: bool = user_repo.email_exists("test@mail.com").await.unwrap();

    assert_eq!(email_exists, false);
}

#[tokio::test]
async fn create_user_succeeds() {
    let pool = get_test_db_pool().await;
    let user_repo = PgUserRepository::new(pool.clone());

    let email = "test@mail.com";

    let email_exists: bool = user_repo.email_exists(email).await.unwrap();
    assert_eq!(email_exists, false);

    let user = User::new(email.to_string());
    user_repo.save(&user).await.unwrap();

    let email_exists: bool = user_repo.email_exists(email).await.unwrap();
    assert_eq!(email_exists, true);
}
