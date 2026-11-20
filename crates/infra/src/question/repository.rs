use domain::question::Question;
use sqlx::{postgres::PgRow, PgPool};

use crate::common::db;

#[derive(Debug, sqlx::FromRow)]
pub struct QuestionRow {
    pub id: i32,
    pub question_text: String,
    pub question_type: String,
    pub chapter_id: i32,
    pub class_id: i32,
    pub subject_id: i32,
    pub dufficulty: i32,
    pub answer_text: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct OptionRow {
    pub id: i32,
    pub question_id: i32,
    pub option_text: String,
    pub is_correct: bool,
    pub order: i32,
}

pub(crate) async fn list_questions(
    pool: &PgPool,
    chapter_id: i32,
) -> domain::Result<Vec<QuestionRow>> {
    let questions: Vec<QuestionRow> = sqlx::query_as::<_, QuestionRow>(r#"
        SELECT id, question_text, answer_text, question_type,chapter_id, class_id, subject_id, difficulty, created_at, updated_at
        FROM questions WHERE chapter_id = $1
        "#)
        .bind(chapter_id)
        .fetch_all(pool)
        .await
        .map_err(db)?;

    Ok(questions)
}

pub(crate) async fn list_question_options(
    pool: &PgPool,
    chapter_id: i32,
) -> domain::Result<Vec<OptionRow>> {
    let options: Vec<OptionRow> = sqlx::query_as(
        r#"SELECT
        qo.id,
        qo.question_id,
        qo.option_text,
        qo.is_correct,
        qo."order" AS "order!"
        FROM question_options qo
    JOIN questions q ON q.id = qo.question_id
        WHERE q.chapter_id = $1 and q.question_type = 'mcq'
   ORDER BY qo.question_id, qo."order"
   "#,
    )
    .bind(chapter_id)
    .fetch_all(pool)
    .await
    .map_err(db)?;

    Ok(options)
}
