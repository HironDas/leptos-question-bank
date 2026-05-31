#[cfg(feature = "ssr")]
use crate::error::QuestionBankError;
use crate::server_function::academic_setting::Chapter;
#[cfg(feature = "ssr")]
use leptos::logging::log;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;

#[server]
pub async fn subject_chapter(subject_id: u32) -> Result<Vec<Chapter>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<Arc<PgPool>>();
        let chapters = sqlx::query!(
            r#"
            SELECT id, title, class_id, subject_id, question_count, "order"  FROM chapters WHERE subject_id = $1 ORDER BY "order" ASC
            "#,
            subject_id as i32
        )
        .try_map(|record| {
            Ok(Chapter {id:record.id as u32,title:record.title,class_id:record.class_id as u32,order:record.order as u32,subject_id:record.subject_id as u32, question_count:record.question_count as u32})
        })
        .fetch_all(&*pool)
        .await?;

        Ok(chapters)
    }
}

#[server]
pub async fn add_chapter(title: String, subject_id: u32) -> Result<Chapter, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<Arc<PgPool>>();

        if title.trim().is_empty() {
            use leptos::logging::log;

            log!("Chapter title is empty");
            return Err(ServerFnError::ServerError(
                "Chapter title cannot be empty".into(),
            ));
        }
        insert_chapter(pool, title, subject_id)
            .await
            //.map(|_| ())
            .map_err(|e| e.into())
    }
}

#[cfg(feature = "ssr")]
pub async fn insert_chapter(
    pool: Arc<PgPool>,
    title: String,
    subject_id: u32,
) -> Result<Chapter, QuestionBankError> {
    use anyhow::Context;
    let order = sqlx::query!(
        r#"
        SELECT COALESCE(MAX("order"), 0) as max_order FROM chapters WHERE subject_id = $1
        "#,
        subject_id as i32
    )
    .fetch_one(&*pool)
    .await
    .context("Failed to fetch max order")?;

    let order = order.max_order.unwrap_or(0) + 1;

    let class_id = sqlx::query!(
        r#"
        SELECT class_id FROM subjects WHERE id = $1 LIMIT 1
        "#,
        subject_id as i32
    )
    .fetch_one(&*pool)
    .await
    .context("Failed to fetch max order")?;

    let class_id = class_id.class_id as i32;

    sqlx::query!(
        r#"
        INSERT INTO chapters (title, class_id, subject_id, "order") VALUES ($1, $2, $3, $4) RETURNING id, title, class_id, subject_id, question_count, "order"
        "#,
        title,
        class_id as i32,
        subject_id as i32,
        order as i32
    )
    .fetch_one(&*pool)
    .await
    .map(|result| {
        log!("Chapter Inserted Successfully! and the id is {:?}", result);
        Ok(Chapter {
            id: result.id as u32,
            title: result.title,
            class_id: result.class_id as u32,
            subject_id: result.subject_id as u32,
            order: result.order as u32,
            question_count: result.question_count as u32,
        })
    })
    .map_err(|e| {
        log!("Failed to insert a chapter: {}", e);
        e
    })
    .context("Something Went wrong on saving chapter")?
}

/// Documentation for [`update_chapter`]
#[server]
pub async fn update_chapter(title: String, chapter_id: u32) -> Result<Chapter, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<Arc<PgPool>>();

        if title.trim().is_empty() {
            use leptos::logging::log;

            log!("Chapter title is empty");
            return Err(ServerFnError::ServerError(
                "Chapter title cannot be empty".into(),
            ));
        }
        edit_chapter(pool, title, chapter_id)
            .await
            //.map(|_| ())
            .map_err(|e| e.into())
    }
}

#[cfg(feature = "ssr")]
pub async fn edit_chapter(
    pool: Arc<PgPool>,
    title: String,
    chapter_id: u32,
) -> Result<Chapter, QuestionBankError> {
    use anyhow::Context;
    sqlx::query!(
            r#"
                UPDATE chapters SET title = $1 WHERE id = $2 returning id, title, class_id, subject_id, question_count, "order"
                "#,
            title,
            chapter_id as i32
        ).map(|record| {
            Chapter {
                id: record.id as u32,
                title: record.title,
                class_id: record.class_id as u32,
                subject_id: record.subject_id as u32,
                order: record.order as u32,
                question_count: record.question_count as u32,
            }
        })
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            log!("Failed to update chapter: {}", e);
            e
        })
        .context("Something Went wrong on updating chapter")
        .map_err(QuestionBankError::from)
}
