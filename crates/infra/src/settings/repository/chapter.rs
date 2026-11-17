use domain::class::Chapter;
use sqlx::{prelude::FromRow, PgPool, Postgres};

use crate::common::db;

#[derive(FromRow)]
struct ChapterRow {
    pub id: i32,
    pub title: String,
    pub subject_id: i32,
    pub class_id: i32,
    pub order: i32,
    pub question_count: i32,
}

impl From<ChapterRow> for Chapter {
    fn from(row: ChapterRow) -> Self {
        Self {
            id: row.id as u32,
            title: row.title,
            order: row.order as u32,
            class_id: row.class_id as u32,
            subject_id: row.subject_id as u32,
            question_count: row.question_count as u32,
        }
    }
}
pub(crate) async fn list_chapters(pool: &PgPool, subject_id: i32) -> domain::Result<Vec<Chapter>> {
    let chapters = sqlx::query_as::<Postgres, ChapterRow>(
        r#"
        SELECT id, title, class_id, subject_id, question_count, "order" FROM chapters
        WHERE subject_id = $1 ORDER BY "order" ASC
        "#,
    )
    .bind(subject_id as i32)
    .fetch_all(pool)
    .await
    .map_err(db)?
    .into_iter()
    .map(Chapter::from)
    .collect::<Vec<Chapter>>();

    Ok(chapters)
}

pub(crate) async fn insert_chapter(
    pool: &PgPool,
    title: &str,
    subject_id: i32,
) -> domain::Result<Chapter> {
    let order: i32 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX("order"), 0) FROM chapters WHERE subject_id = $1
        "#,
    )
    .bind(subject_id as i32)
    .fetch_one(pool)
    .await
    .map_err(db)?;

    let new_order = order + 10;

    let class_id =
        sqlx::query_scalar::<_, i32>(r#"SELECT class_id FROM SUBJECTS WHERE id = $1 LIMIT 1"#)
            .bind(subject_id as i32)
            .fetch_one(pool)
            .await
            .map_err(db)?;
    let chapter = sqlx::query_as::<_, ChapterRow>(
        r#"
        INSERT INTO chapters (title, class_id, subject_id, "order")
        VALUES ($1, $2,$3, $4) RETURN id, title, class_id, subject_id, question_count, "order"
        "#,
    )
    .bind(title)
    .bind(class_id)
    .bind(subject_id as i32)
    .bind(new_order)
    .fetch_one(pool)
    .await
    .map(Chapter::from)
    .map_err(db)?;

    Ok(chapter)
}

pub(crate) async fn update_chapter(pool: &PgPool, title: &str, id: i32) -> domain::Result<Chapter> {
    sqlx::query_as::<_, ChapterRow>(r#"
        UPDATE chapters SET title = $1 WHERE id = $2 returning id, title, class_id, subject_id, question_count, "order"
        "#).bind(title)
    .bind(id as i32)
    .fetch_one(pool)
    .await
    .map(Chapter::from)
    .map_err(db)
}
