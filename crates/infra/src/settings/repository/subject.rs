use domain::{class::Subject, Error};
use sqlx::{prelude::FromRow, PgPool};

use crate::common::db;

#[derive(FromRow)]
struct SubjectRow {
    pub id: i32,
    pub title: String,
    pub class_id: i32,
    pub order: i32,
}

impl From<SubjectRow> for Subject {
    fn from(row: SubjectRow) -> Self {
        Self {
            id: row.id as u32,
            title: row.title,
            class_id: row.class_id as u32,
            order: row.order as u32,
        }
    }
}

pub(crate) async fn list_subjects(pool: &PgPool, class_id: i32) -> domain::Result<Vec<Subject>> {
    let subjects: Vec<Subject> = sqlx::query_as::<_, SubjectRow>(
        r#"
    SELECT id, title, class_id, "order"  FROM subjects WHERE class_id = $1 ORDER BY "order" ASC
    "#,
    )
    .bind(class_id as i32)
    .fetch_all(pool)
    .await
    .map_err(db)?
    .into_iter()
    .map(Subject::from)
    .collect();

    Ok(subjects)
}

pub(crate) async fn insert_subject(
    pool: &PgPool,
    title: &str,
    class_id: i32,
) -> domain::Result<Subject> {
    let order: i32 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX("order"), 0) as max_order FROM subjects WHERE class_id = $1
    "#,
    )
    .bind(class_id)
    .fetch_one(pool)
    .await
    .map_err(db)?;

    let new_order = order + 10;

    let subject: Subject = sqlx::query_as::<_, SubjectRow>(r#"
        INSERT INTO subjects (title, class_id, "order") VALUES ($1, $2, $3) RETURNING id, title, class_id, "order"
    "#)
    .bind(title)
    .bind(class_id as i32)
    .bind(new_order)
    .fetch_one(pool)
    .await
    .map(Subject::from)
    .map_err(db)?;

    Ok(subject)
}

pub(crate) async fn update_subject(pool: &PgPool, title: &str, id: i32) -> domain::Result<Subject> {
    let id = i32::try_from(id).map_err(|_| Error::Internal)?;
    let subject = sqlx::query_as::<_, SubjectRow>(
        r#"
        UPDATE subjects SET title = $1 WHERE id = $2
        RETURNING id, title, class_id, "order"
        "#,
    )
    .bind(title)
    .bind(id)
    .fetch_one(pool)
    .await
    .map(Subject::from)
    .map_err(db)?;

    Ok(subject)
}
