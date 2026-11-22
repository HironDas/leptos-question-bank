use domain::{
    class::UpdateClassInput,
    question::{
        AddQuestionInput, AddQuestionOptionInput, Question, QuestionOption, QuestionType,
        UpdateQuestionInput,
    },
};
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::common::db;

#[derive(Debug, sqlx::FromRow)]
pub struct QuestionRow {
    pub id: i32,
    pub question_text: String,
    pub question_type: String,
    pub chapter_id: i32,
    pub class_id: i32,
    pub subject_id: i32,
    pub difficulty: i32,
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

pub(crate) async fn find_question_by_id(pool: &PgPool, id: i32) -> domain::Result<QuestionRow> {
    sqlx::query_as::<_, QuestionRow>(
        r#"
        SELECT id, question_text, answer_text, question_type,chapter_id, class_id, subject_id, difficulty, created_at, updated_at
        FROM questions WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(db)
}

pub(crate) async fn list_options_by_question_id(
    pool: &PgPool,
    question_id: i32,
) -> domain::Result<Vec<OptionRow>> {
    sqlx::query_as(
        r#"SELECT
        qo.id,
        qo.question_id,
        qo.option_text,
        qo.is_correct,
        qo."order" AS "order!"
        FROM question_options qo
        WHERE qo.question_id = $1
   ORDER BY qo.question_id, qo."order"
   "#,
    )
    .bind(question_id)
    .fetch_all(pool)
    .await
    .map_err(db)
}

pub(crate) fn map_question(
    row: QuestionRow,
    options: Vec<QuestionOption>,
) -> domain::Result<Question> {
    let question_type = match row.question_type.as_str() {
        "mcq" => QuestionType::MCQ,
        "cq" => QuestionType::CQ,
        _ => return Err(domain::Error::Invalid("Unknown question type")),
    };

    Ok(Question {
        id: row.id,
        question_text: row.question_text,
        question_type,
        chapter_id: row.chapter_id,
        class_id: row.class_id,
        subject_id: row.subject_id,
        difficulty: row.difficulty,
        answer_text: row.answer_text,
        created_at: row.created_at.to_string(),
        updated_at: row.updated_at.to_string(),
        options,
    })
}

pub(crate) async fn insert_question(
    tx: &mut sqlx::Transaction<'_, Postgres>,
    input: AddQuestionInput,
) -> domain::Result<QuestionRow> {
    sqlx::query_as::<_, QuestionRow>(r#"
        INSERT INTO questions (question_text, question_type, chapter_id, class_id, subject_id, difficulty, answer_text)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, question_text, question_type, chapter_id, class_id, subject_id, difficulty, answer_text, created_at, updated_at
        "#)
    .bind(input.question_text)
    .bind(input.question_type)
    .bind(input.chapter_id)
    .bind(input.class_id)
    .bind(input.subject_id)
    .bind(input.difficulty)
    .bind(input.answer_text.as_deref())
        .fetch_one(&mut **tx)
        .await
        .map_err(db)
}

pub(crate) async fn insert_options(
    tx: &mut sqlx::Transaction<'_, Postgres>,
    question_id: i32,
    options: &[AddQuestionOptionInput],
) -> domain::Result<Vec<QuestionOption>> {
    if options.is_empty() {
        return Ok(Vec::new());
    }

    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        INSERT INTO question_options (question_id, option_text, is_correct, "order")
        "#,
    );

    builder.push_values(options, |mut b, option| {
        b.push_bind(question_id)
            .push_bind(&option.option_text)
            .push_bind(option.is_correct)
            .push_bind(option.order);
    });

    builder.push(r#"RETURNING id, question_id, option_text, is_correct, "order" "#);

    let rows = builder
        .build_query_as::<OptionRow>()
        .fetch_all(&mut **tx)
        .await
        .map_err(db)?;

    let mut result: Vec<QuestionOption> = rows
        .into_iter()
        .map(|row| QuestionOption {
            id: row.id,
            option_text: row.option_text,
            is_correct: row.is_correct,
            order: row.order,
        })
        .collect();

    result.sort_by_key(|option| option.order);

    Ok(result)
}

pub(crate) async fn increment_chapter_question_count(
    tx: &mut sqlx::Transaction<'_, Postgres>,
    chapter_id: i32,
) -> domain::Result<()> {
    sqlx::query(r#"UPDATE chapters SET question_count = question_count +1 where id= $1"#)
        .bind(chapter_id)
        .execute(&mut **tx)
        .await
        .map_err(db)?;
    Ok(())
}

pub(crate) async fn find_question_chapter(
    tx: &mut sqlx::Transaction<'_, Postgres>,
    question_id: i32,
) -> domain::Result<i32> {
    sqlx::query_scalar(r#"SELECT chapter_id FROM questions WHERE id=$1 FOR UPDATE"#)
        .bind(question_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(db)
}

pub(crate) async fn update_question(
    tx: &mut sqlx::Transaction<'_, Postgres>,
    input: UpdateQuestionInput,
) -> domain::Result<QuestionRow> {
    sqlx::query_as::<_, QuestionRow>(r#"
        UPDATE questions SET question_text = $1, question_type = $2, difficulty = $3, answer_text = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING id, question_text, question_type, chapter_id, class_id, subject_id, difficulty, answer_text, created_at, updated_at
        "#)
    .bind(input.question_text)
    .bind(input.question_type)
    .bind(input.answer_text)
    .bind(input.id)
        .fetch_one(&mut **tx)
        .await
        .map_err(db)
}

pub(crate) async fn delete_old_options(
    tx: &mut sqlx::Transaction<'_, Postgres>,
    question_id: i32,
) -> domain::Result<()> {
    todo!()
}
