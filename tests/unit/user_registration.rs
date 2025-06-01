// /* user registration unit test */
// use mockall::predicate::*;

// use gandalf::adapters::repositories::Error as RepositoryError;
// use gandalf::domain::models::User;
// use gandalf::domain::services::{Error, UserService};

// use crate::mocks::MockUserRepository;

// fn setup_service_with_mock<F>(setup: F) -> UserService<MockUserRepository>
// where
//     F: FnOnce(&mut MockUserRepository),
// {
//     let mut mock_repo = MockUserRepository::new();
//     setup(&mut mock_repo);
//     UserService::new(mock_repo)
// }

// #[tokio::test]
// async fn register_user_successful() {
//     let email = "test@example.com";
//     let password = "password123";

//     let service = setup_service_with_mock(|mock| {
//         mock.expect_email_exists()
//             .with(eq(email))
//             .returning(|_| Ok(false));

//         mock.expect_save()
//             .withf(move |user: &User| {
//                 user.email == email && user.password_hash == Some(password.to_string())
//             })
//             .returning(|_| Ok(()));
//     });

//     let result = service
//         .register_user(email.to_string(), password.to_string())
//         .await;

//     assert!(result.is_ok(), "User should be registered successfully");
//     let user = result.unwrap();
//     assert_eq!(user.email, email);
// }

// #[tokio::test]
// async fn register_user_fails_when_email_exists() {
//     let email = "existing@example.com";
//     let password = "password123";

//     let service = setup_service_with_mock(|mock| {
//         mock.expect_email_exists()
//             .with(eq(email))
//             .returning(|_| Ok(true));

//         mock.expect_save().times(0);
//     });

//     let result = service
//         .register_user(email.to_string(), password.to_string())
//         .await;

//     assert!(matches!(result, Err(Error::UserAlreadyExists)));
// }

// #[tokio::test]
// async fn register_user_fails_on_repository_error() {
//     let email = "test@example.com";

//     let service = setup_service_with_mock(|mock| {
//         mock.expect_email_exists().with(eq(email)).returning(|_| {
//             Err(RepositoryError::ConnectionError(
//                 "Database connection error".into(),
//             ))
//         });
//     });

//     let result = service
//         .register_user(email.to_string(), "any-password".to_string())
//         .await;

//     assert!(
//         matches!(result, Err(Error::RepositoryError(_))),
//         "Expected RepositoryError, got: {:?}",
//         result
//     );
// }

// #[tokio::test]
// async fn register_user_with_empty_email_is_ok() {
//     let email = "";
//     let password = "password123";

//     let service = setup_service_with_mock(|mock| {
//         mock.expect_email_exists()
//             .with(eq(email))
//             .returning(|_| Ok(false));

//         mock.expect_save()
//             .withf(move |user: &User| {
//                 user.email.is_empty() && user.password_hash == Some(password.to_string())
//             })
//             .returning(|_| Ok(()));
//     });

//     let result = service
//         .register_user(email.to_string(), password.to_string())
//         .await;

//     assert!(
//         result.is_ok(),
//         "Registration with empty email should succeed"
//     );
//     let user = result.unwrap();
//     assert_eq!(user.email, "");
//     assert_eq!(user.password_hash, Some(password.to_string()));
// }
