#[cfg(feature = "test-fullstack")]
pub struct TestApp {
    pub address: String,
    // pub db_pool: sqlx::PgPool,
    pub port: u16,
}

#[cfg(feature = "test-fullstack")]
pub async fn spawn_app() -> TestApp {
    use leptos_question_bank::run;
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // let _ = tokio::spawn(run(listener));

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        // db_pool: db_pool,
        port: port,
    }
}
