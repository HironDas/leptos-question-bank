use crate::server_function::academic_setting::Chapter;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;
use leptos::prelude::*;

#[server]
pub async fn subject_chapter(subject_id: u32) -> Result<Vec<Chapter>, ServerFnError> {
    #[cfg(feature = "ssr")]{
        let pool = expect_context::<Arc<PgPool>>();
        let chapters = sqlx::query!(
            r#"
            SELECT id, title, class_id, subject_id, "order"  FROM chapters WHERE subject_id = $1 ORDER BY "order" ASC
            "#,
            subject_id as i32
        )
        .try_map(|record| {
            Ok(Chapter {id:record.id as u32,title:record.title,class_id:record.class_id as u32,order:record.order as u32,subject_id:record.subject_id as u32 })
        })
        .fetch_all(&*pool)
        .await?;

        Ok(chapters)
    }
}