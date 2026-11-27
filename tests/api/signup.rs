use infra::user::UserService;

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn signup_return_200_for_valid_input() {
    use crate::helpers::spawn_app;
    use domain::new_user::NewUser;
    use infra::user::UserService;
    use leptos_question_bank::server_function::signup::dto::User;
    use pwhash::bcrypt;

    // let _runtime = leptos_reactive::create_runtime();
    // After (fixes the error during cargo test)
    // let mut conf =
    //     leptos::config::get_configuration(Some("Cargo.toml")).expect("Failed to get configuration");

    let app = spawn_app().await;
    // conf.leptos_options.site_addr =
    //     std::net::SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), app.port);

    // println!("CONFIG: {:?}", conf);
    // leptos::provide_context(Extension(app.db_pool.clone()));

    let user = User {
        username: "hiron".to_string(),
        email: "hcdas@gmail.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };

    let ctx = app.ctx;
    let service = UserService::from(ctx);
    let new_user: NewUser = user.clone().try_into().unwrap();

    // leptos::context::provide_context(pool.clone());

    service
        .signup(new_user)
        .await
        .expect("Signup action failed to run");

    let user = sqlx::query!(
        "SELECT username, active, password_hash FROM users WHERE email = $1",
        user.email
    )
    .fetch_one(&app.pool)
    .await
    .expect("Failed to fetch user");

    assert_eq!(user.username, "hiron");
    assert_eq!(user.active, Some(false));
    assert!(bcrypt::verify("Hiron@123", &user.password_hash));
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn signup_with_duplicate_username_fails() {
    use domain::new_user::NewUser;
    use leptos_question_bank::server_function::signup::dto::User;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = UserService::from(app.ctx);

    let user = User {
        username: "duplicate".to_string(),
        email: "first@example.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };
    let new_user: NewUser = user.clone().try_into().unwrap();

    service
        .signup(new_user)
        .await
        .expect("First signup should succeed");

    let duplicate = User {
        username: "duplicate".to_string(),
        email: "second@example.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };
    let new_user: NewUser = duplicate.try_into().unwrap();
    let result = service.signup(new_user).await;
    assert!(result.is_err());
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn signup_with_duplicate_email_fails() {
    use leptos_question_bank::server_function::signup::dto::User;

    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let service = UserService::from(app.ctx);

    let user = User {
        username: "user_one".to_string(),
        email: "same@example.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };
    let new_user = user.try_into().unwrap();

    service
        .signup(new_user)
        .await
        .expect("First signup should succeed");

    let duplicate = User {
        username: "user_two".to_string(),
        email: "same@example.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };
    let new_user = duplicate.try_into().unwrap();
    let result = service.signup(new_user).await;
    assert!(result.is_err());
}
