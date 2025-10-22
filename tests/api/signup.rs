#[cfg(feature = "test-fullstack")]
#[tokio::test]
async fn signup_return_200_for_valid_input() {
    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/signup", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
