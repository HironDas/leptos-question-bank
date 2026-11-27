use std::net::IpAddr;

#[cfg(feature = "test-fullstack")]
use infra::user::UserService;
use leptos_question_bank::server_function::login::dto::LoginCredential;
#[cfg(feature = "test-fullstack")]
use leptos_question_bank::server_function::signup::dto::User;

#[cfg(feature = "test-fullstack")]
use crate::helpers::TestApp;

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn login_return_session_for_valid_input() {
    use leptos_question_bank::server_function::login::dto::LoginCredential;

    let app = create_user().await;
    let service = UserService::from(app.ctx);
    let login_user = LoginCredential {
        id: "hiron".to_string(),
        password: "Hiron@123".to_string(),
    };

    let login_user = login_user.try_into().expect("Parsing error");
    let ip: IpAddr = app.address.parse().expect("Invalid Ip Address");
    let session = service
        .login(login_user, ip)
        .await
        .expect("Failed to get the user id");

    assert_eq!(session.len(), 32);
    // search with email
    let login_user = LoginCredential {
        id: "hcdas@gmail.com".to_string(),
        password: "Hiron@123".to_string(),
    };

    let login_user = login_user.try_into().expect("Parsing error");
    let session = service
        .login(login_user, ip)
        .await
        .expect("Failed to get the user id");
    assert_eq!(session.len(), 32);
}

#[cfg(feature = "test-fullstack")]
async fn create_user() -> TestApp {
    use crate::helpers::spawn_app;
    use domain::new_user::NewUser;

    let app = spawn_app().await;

    let user = User {
        username: "hiron".to_string(),
        email: "hcdas@gmail.com".to_string(),
        password: "Hiron@123".to_string(),
        confirm_password: "Hiron@123".to_string(),
    };

    let new_user: NewUser = user.clone().try_into().unwrap();

    let service = UserService::from(app.ctx.clone());

    service
        .signup(new_user)
        .await
        .expect("Signup action failed to run");
    app
}

#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn login_with_wrong_password_fails() {
    use domain::login::Login;
    use infra::user::UserService;

    let app = create_user().await;
    let service = UserService::from(app.ctx);
    let login_user = LoginCredential {
        id: "hiron".to_string(),
        password: "Wrong@123".to_string(),
    };
    let login_user: Login = login_user.try_into().expect("Parsing error");
    let ip: IpAddr = app.address.parse().expect("Invalid Ip Address");
    let result = service.login(login_user, ip).await;
    assert!(result.is_err());
}
