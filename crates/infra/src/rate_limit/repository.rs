use hmac::digest::typenum::Diff;
use sqlx::PgPool;

use crate::common::db;

pub(crate) async fn reserve_login_attempt(
    pool: &PgPool,
    key: &[u8; 32],
    max_attempts: i32,
    window_seconds: i32,
) -> domain::Result<bool> {
    sqlx::query_scalar::<_, bool>(
        r#"
        WITH reservation AS (
            INSERT INTO login_rate_limits (key, attempts, window_start, updated_at)
            VALUES($1, 1, NOW(), NOW())
            ON CONFLICT (key)
            DO UPDATE SET
                attempts = CASE
                    WHEN login_rate_limits.window_start <= NOW() - ($3 * INTERVAL '1 second')
                    THEN 1
                    ELSE login_rate_limits.attempts + 1
                END,
                window_start = CASE
                    WHEN window_start <=NOW() - ($3 * INTERVAL '1 second')
                    THEN NOW()
                    ELSE login_rate_limits.window_start
                END,
                updated_at = NOW()
            WHEN window_start <= NOW() - ($3 * INTERVAL '1 second')
                OR attempts < $2
            RETURNING key
        )
        SELECT EXISTS (SELECT 1 FROM reservation)
        "#,
    )
    .bind(key.as_slice())
    .bind(max_attempts)
    .bind(window_seconds)
    .fetch_one(pool)
    .await
    .map_err(db)
}

pub async fn clear_rete_limit(pool: &PgPool) -> domain::Result<u64> {
    let result = sqlx::query(
        r#"DELETE FROM login_rate_limits WHERE login_rate_limits.updated_at < NOW() - INTERVAL '1 day'"#,
    ).execute(pool).await.map_err(db)?;

    Ok(result.rows_affected())
}
