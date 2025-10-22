#[cfg(feature = "test-fullstack")]
#[tokio::test] 
async fn health_check_works(){
    use crate::helpers::spawn_app;

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    println!("Testing health check at {}", &app.address);

    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert!(response.status().is_success());
    assert!(response.text().await.unwrap().contains(""));
}