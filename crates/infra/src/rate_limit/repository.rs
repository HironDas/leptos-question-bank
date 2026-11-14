use sqlx::PgPool;

pub async fn reserve_login_attempt(
    pool: &PgPool,
    key: &[u8; 32],
    max_attempts: i32,
) -> Result<bool, sqlx::Error> {
    let allowed = sqlx::query_scalar::<_, bool>(
        r#"
        INSERT INTO login_rate_limits
            (key, attempts, window_start, updated_at)
        VALUES ($1, 1, NOW(), NOW())
        ON CONFLICT (key)
        DO UPDATE SET
            attempts = CASE
                WHEN login_rate_limits.window_start
                     <= NOW() - INTERVAL '15 minutes'
                THEN 1
                WHEN login_rate_limits.attempts < $2
                THEN login_rate_limits.attempts + 1
                ELSE login_rate_limits.attempts
            END,
            window_start = CASE
                WHEN login_rate_limits.window_start
                     <= NOW() - INTERVAL '15 minutes'
                THEN NOW()
                ELSE login_rate_limits.window_start
            END,
            updated_at = NOW()
        RETURNING attempts <= $2
        "#,
    )
    .bind(key.as_slice())
    .bind(max_attempts)
    .fetch_one(pool)
    .await?;

    Ok(allowed)
}
