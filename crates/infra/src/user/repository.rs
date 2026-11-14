use domain::new_user::NewUser;
use sqlx::{types::uuid, Executor, PgPool, Postgres, Transaction};

use crate::common::db;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    // username: String,
    pub password_hash: String,
}

pub(crate) async fn get_user_by_email(
    pool: &PgPool,
    email: impl AsRef<str>,
) -> domain::Result<User> {
    sqlx::query_as("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(email.as_ref())
        .fetch_one(&*pool)
        .await
        .map_err(db)
}

pub(crate) async fn get_user_by_username(
    pool: &PgPool,
    username: impl AsRef<str>,
) -> domain::Result<User> {
    sqlx::query_as("SELECT id, password_hash FROM users WHERE username = $1")
        .bind(username.as_ref())
        .fetch_one(&*pool)
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

pub(crate) async fn store_session(pool: &PgPool, user_id: i32) -> domain::Result<String> {
    let date = chrono::Local::now() + chrono::Duration::days(7);
    let session_id = uuid::Uuid::new_v4();

    sqlx::query("UPDATE INTO session(user_id, session_token, expires_at) VALUES ($1, $2, $3)")
        .bind(&user_id)
        .bind(&session_id)
        .bind(&date.naive_local())
        .execute(pool)
        .await
        .map_err(db)?;
    Ok(session_id.to_string())
}

pub(crate) async fn insert_new_user<'e, E>(executor: E, user: NewUser) -> domain::Result<()>
where
    E: Executor<'e, Database = Postgres>,
{
    let password_hash = user.hash_password();

    sqlx::query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2,$3)")
        .bind(user.username.as_ref())
        .bind(user.email.as_ref())
        .bind(&password_hash)
        .execute(executor)
        .await
        .map_err(db)?;
    Ok(())
}

pub(crate) async fn check_user(pool: &PgPool, username: &str) -> domain::Result<bool> {
    let exist: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(db)?;

    Ok(exist)
}

pub(crate) async fn check_email(pool: &PgPool, email: &str) -> domain::Result<bool> {
    let exist: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(db)?;

    Ok(exist)
}
