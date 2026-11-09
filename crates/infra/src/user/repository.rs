#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    // username: String,
    pub password_hash: String,
}

pub(crate) async fn get_user_by_email(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    email: String,
) -> domain::Result<User> {
    todo!()
}
