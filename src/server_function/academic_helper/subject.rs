use crate::{error::QuestionBankError, server_function::academic_setting::Subject as AcademicSubject};
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;
use leptos::{logging::log, prelude::*};

#[server]
pub async fn subject(class_id: u32) -> Result<Vec<AcademicSubject>, ServerFnError> {
    #[cfg(feature = "ssr")]{
        let pool = expect_context::<Arc<PgPool>>();
        let subjects = sqlx::query!(
            r#"
            SELECT id, title, class_id, "order"  FROM subjects WHERE class_id = $1 ORDER BY "order" ASC
            "#,
            class_id as i32
        )
        .try_map(|record| {
            Ok(AcademicSubject {
                id: record.id as u32,
                title: record.title,
                class_id: record.class_id as u32,
                order: record.order as u32,
            })
        })
        .fetch_all(&*pool)
        .await?;

        Ok(subjects)
    }
}

#[server] 
pub async fn add_subject(title: String, class_id: u32)->Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]{
        let pool = expect_context::<Arc<PgPool>>();

        if title.trim().is_empty() {
            log!("Subject title is empty");
            return Err(ServerFnError::ServerError("Subject title cannot be empty".into()));
        }
        insert_subject(pool, title, class_id).await.map(|_|()).map_err(|e| e.into())
    }
    // Ok(())
}

#[cfg(feature = "ssr")]
pub async fn insert_subject(pool: Arc<PgPool>, title: String, class_id: u32) -> Result<(), QuestionBankError> {
    use anyhow::Context;
    let order = sqlx::query!(
        r#"
        SELECT COALESCE(MAX("order"), 0) as max_order FROM subjects WHERE class_id = $1
        "#,
        class_id as i32
    )
    .fetch_one(&*pool)
    .await.context("Failed to fetch max order")?;

    let order = order.max_order.unwrap_or(0) + 1;

    sqlx::query!(
        r#"
        INSERT INTO subjects (title, class_id, "order") VALUES ($1, $2, $3)
        "#,
        title,
        class_id as i32,
        order as i32
    )
    .execute(&*pool)
    .await
    .map(|result|{
        log!("Subject Inserted Successfully! and the id is {:?}", result);
        Ok(())
    })
    .map_err(|e|{
        log!("Failed to insert subject: {}", e);
        e
    }).context("Something Went wrong on saving subject")?    
}