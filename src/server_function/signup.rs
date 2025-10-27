use leptos::prelude::*;

#[server]
pub async fn signup(
    username: String,
    email: String,
    password: String,
    confirm_password: String,
) -> Result<(), ServerFnError> {
    leptos::logging::log!("Signup is clicked --");
    use crate::domain::new_user::NewUser;
    let user: NewUser = NewUser::new(username, email, password, confirm_password);
    let password_hash = user.hash_password();
    
    #[cfg(feature = "ssr")]{

        use sqlx::PgPool;
        use std::sync::Arc;
    
        let pool = expect_context::<Arc<PgPool>>();
    
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
    }
    Ok(())
}
