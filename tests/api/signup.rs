#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn signup_return_200_for_valid_input() {
    use axum::Extension;
    use leptos_question_bank::server_function::signup::{signup_action, User};
    use pwhash::bcrypt;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;

    leptos::provide_context(Extension(app.db_pool.clone()));

    let user = User {
        username: "hiron".to_string(),
        email: "hcdas@gmail.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };

    signup_action(user.clone())
        .await
        .expect("Signup action failed to run");

    let user = sqlx::query!(
        "SELECT username, active, password_hash FROM users WHERE email = $1",
        user.email
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch user");

    assert_eq!(user.username, "hiron");
    assert_eq!(user.active, Some(false));
    assert!(bcrypt::verify("Hiron@123", &user.password_hash));
}
