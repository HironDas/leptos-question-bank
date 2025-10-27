use leptos::prelude::*;

#[cfg(feature = "ssr")]
#[server]
pub async fn signup(
    username: String,
    email: String,
    password: String,
    confirm_password: String,
) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use std::sync::Arc;

    print!("Singn up function is called!!");
    use crate::domain::new_user::NewUser;

    let pool = expect_context::<Arc<PgPool>>();
    let user: NewUser = NewUser::new(username, email, password, confirm_password);
    let password_hash = user.hash_password();

    sqlx::query(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.username.as_ref())
    .bind(user.email.as_ref())
    .bind(&password_hash)
    .execute(&*pool)
    .await?;
    Ok(())
}
