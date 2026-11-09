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
