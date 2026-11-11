use sqlx::types::uuid;

use crate::common::db;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    // username: String,
    pub password_hash: String,
}

pub(crate) async fn get_user_by_email(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    email: impl AsRef<str>,
) -> domain::Result<User> {
    sqlx::query_as("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(email.as_ref())
        .fetch_one(&mut **tx)
        .await
        .map_err(db)
}

pub(crate) async fn get_user_by_username(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    username: impl AsRef<str>,
) -> domain::Result<User> {
    sqlx::query_as("SELECT id, password_hash FROM users WHERE username = $1")
        .bind(username.as_ref())
        .fetch_one(&mut **tx)
        .await
        .map_err(db)
}

pub(crate) async fn delete_expired_session(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> domain::Result<sqlx::postgres::PgQueryResult> {
    sqlx::query("DELETE FROM sessions WHERE expires_at < $1")
        .bind(&chrono::Local::now().naive_local())
        .execute(&mut **tx)
        .await
        .map_err(db)
}

pub(crate) async fn store_session(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i32,
) -> domain::Result<String> {
    let date = chrono::Local::now() + chrono::Duration::days(7);
    let session_id = uuid::Uuid::new_v4();

    sqlx::query("UPDATE INTO session(user_id, session_token, expires_at) VALUES ($1, $2, $3)")
        .bind(&user_id)
        .bind(&session_id)
        .bind(&date.naive_local())
        .execute(&mut **tx)
        .await
        .map_err(db)?;
    Ok(session_id.to_string())
}
