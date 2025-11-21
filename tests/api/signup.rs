#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn signup_return_200_for_valid_input() {
    use leptos_question_bank::server_function::signup::{insert_new_user, User};
    use pwhash::bcrypt;
    use std::{
        //net::{IpAddr, Ipv4Addr},
        sync::Arc,
    };

    use crate::helpers::spawn_app;
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

    let pool = Arc::new(app.db_pool);
    let new_user = user.clone().try_into().unwrap();

    // leptos::context::provide_context(pool.clone());

    insert_new_user(new_user, pool.clone())
        .await
        .expect("Signup action failed to run");

    let user = sqlx::query!(
        "SELECT username, active, password_hash FROM users WHERE email = $1",
        user.email
    )
    .fetch_one(&*pool)
    .await
    .expect("Failed to fetch user");

    assert_eq!(user.username, "hiron");
    assert_eq!(user.active, Some(false));
    assert!(bcrypt::verify("Hiron@123", &user.password_hash));
}
