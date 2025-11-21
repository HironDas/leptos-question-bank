#[cfg(feature = "test-fullstack")]
use crate::helpers::TestApp;

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn login_return_a_200_for_valid_input() {
    use std::sync::Arc;

    use leptos_question_bank::server_function::{login::user_logged_in, login::LoginCredential};

    let app = create_user().await;
    let pool = Arc::new(app.db_pool);
    let login_user = LoginCredential {
        id: "hiron".to_string(),
        password: "Hiron@123".to_string(),
    };

    let login_user = login_user.try_into().expect("Parsing error");
    let id = user_logged_in(login_user, pool.clone())
        .await
        .expect("Failed to get the user id");

    assert_eq!(id, 1);

    // search with email
    let login_user = LoginCredential {
        id: "hcdas@gmail.com".to_string(),
        password: "Hiron@123".to_string(),
    };

    let login_user = login_user.try_into().expect("Parsing error");
    let id = user_logged_in(login_user, pool.clone())
        .await
        .expect("Failed to get the user id");
    assert_eq!(id, 1);
}

#[cfg(feature = "test-fullstack")]
async fn create_user() -> TestApp {
    use std::sync::Arc;

    use leptos_question_bank::server_function::{signup::insert_new_user, signup::User};

    use crate::helpers::spawn_app;

    let app = spawn_app().await;

    let user = User {
        username: "hiron".to_string(),
        email: "hcdas@gmail.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };

    let new_user = user.clone().try_into().unwrap();

    let pool = Arc::new(app.db_pool.clone());

    insert_new_user(new_user, pool.clone())
        .await
        .expect("Signup action failed to run");
    app
}
