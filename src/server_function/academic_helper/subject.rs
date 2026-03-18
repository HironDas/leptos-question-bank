use crate::{
    error::QuestionBankError, server_function::academic_setting::Subject as AcademicSubject,
};
use leptos::{logging::log, prelude::*};
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;

#[server]
pub async fn subject(class_id: u32) -> Result<Vec<AcademicSubject>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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
pub async fn add_subject(title: String, class_id: u32) -> Result<AcademicSubject, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<Arc<PgPool>>();

        if title.trim().is_empty() {
            log!("Subject title is empty");
            return Err(ServerFnError::ServerError(
                "Subject title cannot be empty".into(),
            ));
        }
        insert_subject(pool, title, class_id)
            .await
            .map(|subject| subject)
            .map_err(|e| e.into())
    }
    // Ok(())
}

#[cfg(feature = "ssr")]
pub async fn insert_subject(
    pool: Arc<PgPool>,
    title: String,
    class_id: u32,
) -> Result<AcademicSubject, QuestionBankError> {
    use anyhow::Context;
    let order = sqlx::query!(
        r#"
        SELECT COALESCE(MAX("order"), 0) as max_order FROM subjects WHERE class_id = $1
        "#,
        class_id as i32
    )
    .fetch_one(&*pool)
    .await
    .context("Failed to fetch max order")?;

    let order = order.max_order.unwrap_or(0) + 1;

    sqlx::query!(
        r#"
        INSERT INTO subjects (title, class_id, "order") VALUES ($1, $2, $3) RETURNING id, title, class_id, "order"
        "#,
        title,
        class_id as i32,
        order as i32
    )
    .fetch_one(&*pool)
    .await
    .map(|result| {
        log!("Subject Inserted Successfully! and the id is {:?}", result);
        Ok(AcademicSubject {
            id: result.id as u32,
            title: result.title,
            class_id: result.class_id as u32,
            order: result.order as u32,
        })
    })
    .map_err(|e| {
        log!("Failed to insert subject: {}", e);
        e
    })
    .context("Something Went wrong on saving subject")?
}

#[server]
pub async fn update_subject(
    title: String,
    subject_id: u32,
) -> Result<AcademicSubject, ServerFnError> {
    if title.trim().is_empty() {
        log!("Subject title is empty");
        return Err(ServerFnError::ServerError(
            "Subject title cannot be empty".into(),
        ));
    } else {
        #[cfg(feature = "ssr")]
        {
            use std::sync::Arc;
            let pool = expect_context::<Arc<PgPool>>();
            edit_subject(pool, title, subject_id)
                .await
                .map_err(|e| e.into())
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn edit_subject(
    pool: Arc<PgPool>,
    title: String,
    subject_id: u32,
) -> Result<AcademicSubject, QuestionBankError> {
    use anyhow::Context;
    sqlx::query!(
        r#"
        UPDATE subjects SET title = $1 WHERE id = $2 RETURNING id, title, class_id, "order"
        "#,
        title,
        subject_id as i32,
    )
    .fetch_one(&*pool)
    .await
    .map(|result| {
        log!("Subject Updated Successfully! and the id is {:?}", result);
        Ok(AcademicSubject {
            id: result.id as u32,
            title: result.title,
            class_id: result.class_id as u32,
            order: result.order as u32,
        })
    })
    .map_err(|e| {
        log!("Failed to update subject: {}", e);
        e
    })
    .context("Something Went wrong on updating subject")?
}
