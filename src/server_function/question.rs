use crate::domain::question::{AddQuestionInput, Question, QuestionOption, QuestionType};
use leptos::prelude::*;

#[server]
pub async fn get_questions(chapter_id: u32) -> Result<Vec<Question>, ServerFnError> {
    let mut questions = Vec::new();
    #[cfg(feature = "ssr")]
    {
        use sqlx::PgPool;
        use std::sync::Arc;

        let pool = expect_context::<Arc<PgPool>>();

        let rows = sqlx::query!(
            r#"
            SELECT id, question_text, question_type, chapter_id, class_id, subject_id,
                   "order", answer_text, created_at
            FROM questions
            WHERE chapter_id = $1
            ORDER BY "order" ASC
            "#,
            chapter_id as i32
        )
        .fetch_all(&*pool)
        .await?;

        for q in rows {
            let question_type = match q.question_type.as_str() {
                "objective" => QuestionType::Objective,
                "subjective" => QuestionType::Subjective,
                _ => QuestionType::Subjective,
            };

            let options = if q.question_type == "objective" {
                let option_rows = sqlx::query!(
                    r#"
                    SELECT id, option_text, is_correct, "order"
                    FROM question_options
                    WHERE question_id = $1
                    ORDER BY "order" ASC
                    "#,
                    q.id
                )
                .fetch_all(&*pool)
                .await?;

                option_rows
                    .into_iter()
                    .map(|o| QuestionOption {
                        id: o.id as u32,
                        option_text: o.option_text,
                        is_correct: o.is_correct,
                        order: o.order as u32,
                    })
                    .collect()
            } else {
                Vec::new()
            };

            questions.push(Question {
                id: q.id as u32,
                question_text: q.question_text,
                question_type,
                chapter_id: q.chapter_id as u32,
                class_id: q.class_id as u32,
                subject_id: q.subject_id as u32,
                order: q.order as u32,
                answer_text: q.answer_text,
                created_at: q.created_at.to_string(),
                options,
            });
        }
    }
    Ok(questions)
}

#[server]
pub async fn add_question(input: AddQuestionInput) -> Result<Question, ServerFnError> {
    use validator::Validate;
    if let Err(errors) = input.validate() {
        leptos::logging::log!("Validation errors: {:?}", errors);
        return Err(ServerFnError::ServerError("Invalid input data".into()));
    }

    #[cfg(feature = "ssr")]
    {
        use sqlx::PgPool;
        use std::sync::Arc;
        let pool = expect_context::<Arc<PgPool>>();
        insert_question(pool, input).await.map_err(|e| e.into())
    }
}

#[cfg(feature = "ssr")]
pub async fn insert_question(
    pool: std::sync::Arc<sqlx::PgPool>,
    input: AddQuestionInput,
) -> Result<Question, crate::error::QuestionBankError> {
    use anyhow::Context;

    // Get max order for this chapter
    let max_order = sqlx::query!(
        r#"
        SELECT COALESCE(MAX("order"), 0) as max_order
        FROM questions
        WHERE chapter_id = $1
        "#,
        input.chapter_id as i32
    )
    .fetch_one(&*pool)
    .await
    .context("Failed to fetch max order")?;

    let new_order = max_order.max_order.unwrap_or(0) + 1;

    let question_type_str = input.question_type.as_str();

    // Insert the question
    let question_row = sqlx::query!(
        r#"
        INSERT INTO questions (question_text, question_type, chapter_id, class_id, subject_id, "order", answer_text)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, question_text, question_type, chapter_id, class_id, subject_id, "order", answer_text, created_at
        "#,
        input.question_text,
        question_type_str,
        input.chapter_id as i32,
        input.class_id as i32,
        input.subject_id as i32,
        new_order as i32,
        input.answer_text.as_deref(),
    )
    .fetch_one(&*pool)
    .await
    .context("Failed to insert question")?;

    let question_id = question_row.id;

    // Insert options for objective questions
    let mut options = Vec::new();
    if question_type_str == "objective" {
        for option_input in &input.options {
            let option_row = sqlx::query!(
                r#"
                INSERT INTO question_options (question_id, option_text, is_correct, "order")
                VALUES ($1, $2, $3, $4)
                RETURNING id, option_text, is_correct, "order"
                "#,
                question_id,
                option_input.option_text,
                option_input.is_correct,
                option_input.order as i32,
            )
            .fetch_one(&*pool)
            .await
            .context("Failed to insert question option")?;

            options.push(QuestionOption {
                id: option_row.id as u32,
                option_text: option_row.option_text,
                is_correct: option_row.is_correct,
                order: option_row.order as u32,
            });
        }
    }

    // Update question count on the chapter
    let _ = sqlx::query!(
        r#"
        UPDATE chapters SET question_count = question_count + 1 WHERE id = $1
        "#,
        input.chapter_id as i32
    )
    .execute(&*pool)
    .await;

    let question_type = match question_row.question_type.as_str() {
        "objective" => QuestionType::Objective,
        "subjective" => QuestionType::Subjective,
        _ => QuestionType::Subjective,
    };

    Ok(Question {
        id: question_row.id as u32,
        question_text: question_row.question_text,
        question_type,
        chapter_id: question_row.chapter_id as u32,
        class_id: question_row.class_id as u32,
        subject_id: question_row.subject_id as u32,
        order: question_row.order as u32,
        answer_text: question_row.answer_text,
        created_at: question_row.created_at.to_string(),
        options,
    })
}
