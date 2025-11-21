use leptos_question_bank::configuration::DatabaseSettings;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[cfg(feature = "test-fullstack")]
pub struct TestApp {
    pub address: String,
    pub db_pool: sqlx::PgPool,
    pub _port: u16,
}

#[cfg(feature = "test-fullstack")]
pub async fn spawn_app() -> TestApp {
    let _runtime = leptos_reactive::create_runtime();

    use leptos_question_bank::{configuration::get_configuration, Application};

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        c.database.database_name = uuid::Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };
    let db_pool = configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build server");
    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application_port);

    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: db_pool, //get_connection_pool(&configuration.database),
        _port: application_port,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = sqlx::PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations");

    connection_pool
}
