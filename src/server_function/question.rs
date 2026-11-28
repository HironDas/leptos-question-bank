use domain::question::{AddQuestionInput, Question, UpdateQuestionInput};
use leptos::prelude::*;

use crate::error::AppError;

#[server]
pub async fn get_questions(chapter_id: i32) -> Result<Vec<Question>, AppError> {
    #[cfg(feature = "ssr")]
    {
        use domain::question::QuestionOption;
        use infra::{common::ServiceContext, question::QuestionService};

        use crate::util::preview_markdown::render_markdown;

        let ctx = expect_context::<ServiceContext>();
        let question_service = QuestionService::from(ctx);

        let questions = question_service
            .load_questions(chapter_id)
            .await?
            .into_iter()
            .map(|q| Question {
                id: q.id,
                question_text: render_markdown(&q.question_text),
                answer_text: match q.answer_text {
                    Some(txt) => Some(render_markdown(&txt)),
                    None => None,
                },
                options: q
                    .options
                    .into_iter()
                    .map(|o| QuestionOption {
                        id: o.id,
                        option_text: render_markdown(&o.option_text),
                        is_correct: o.is_correct,
                        order: o.order,
                    })
                    .collect::<Vec<QuestionOption>>(),
                question_type: q.question_type,
                chapter_id: q.chapter_id,
                class_id: q.class_id,
                subject_id: q.subject_id,
                difficulty: q.difficulty,
                created_at: q.created_at,
                updated_at: q.updated_at,
            })
            .collect::<Vec<Question>>();
        Ok(questions)
    }
}

#[server]
pub async fn get_question_by_id(id: i32) -> Result<Question, AppError> {
    // let mut questions = Vec::new();
    #[cfg(feature = "ssr")]
    {
        use infra::{common::ServiceContext, question::QuestionService};

        let ctx = expect_context::<ServiceContext>();
        let question_service = QuestionService::from(ctx);

        question_service
            .find_quesiton(id)
            .await
            .map_err(|err| err.into())
    }
}

#[server]
pub async fn add_question(input: AddQuestionInput) -> Result<Question, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::{common::ServiceContext, question::QuestionService};

        let ctx = expect_context::<ServiceContext>();
        let question_service = QuestionService::from(ctx);

        question_service
            .add_question(input)
            .await
            .map_err(|err| err.into())
    }
}

#[server]
pub async fn edit_question(input: UpdateQuestionInput) -> Result<Question, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::{common::ServiceContext, question::QuestionService};
        use leptos::logging::log;

        let ctx = expect_context::<ServiceContext>();
        let question_service = QuestionService::from(ctx);

        log!("update Input: {:#?}", input);
        question_service
            .edit_question(input)
            .await
            .map_err(|err| err.into())
    }
}

// #[cfg(feature = "ssr")]
// pub async fn insert_question(
//     pool: std::sync::Arc<sqlx::PgPool>,
//     input: AddQuestionInput,
// ) -> Result<Question, crate::error::QuestionBankError> {
//     use anyhow::Context;
//     let question_type_str = input.question_type.as_str();

//     // Insert the question
//     let question_row = sqlx::query!(
//         r#"
//         INSERT INTO questions (question_text, question_type, chapter_id, class_id, subject_id, difficulty, answer_text)
//         VALUES ($1, $2, $3, $4, $5, $6, $7)
//         RETURNING id, question_text, question_type, chapter_id, class_id, subject_id, difficulty, answer_text, created_at, updated_at
//         "#,
//         input.question_text,
//         question_type_str,
//         input.chapter_id as i32,
//         input.class_id as i32,
//         input.subject_id as i32,
//         input.difficulty as i32,
//         input.answer_text.as_deref(),
//     )
//     .fetch_one(&*pool)
//     .await
//     .context("Failed to insert question")?;

//     let question_id = question_row.id;

//     // Insert options for mcq questions
//     let mut options = Vec::new();
//     if question_type_str == "mcq" {
//         for option_input in &input.options {
//             let option_row = sqlx::query!(
//                 r#"
//                 INSERT INTO question_options (question_id, option_text, is_correct, "order")
//                 VALUES ($1, $2, $3, $4)
//                 RETURNING id, option_text, is_correct, "order"
//                 "#,
//                 question_id,
//                 option_input.option_text,
//                 option_input.is_correct,
//                 option_input.order as i32,
//             )
//             .fetch_one(&*pool)
//             .await
//             .context("Failed to insert question option")?;

//             options.push(QuestionOption {
//                 id: option_row.id as u32,
//                 option_text: option_row.option_text,
//                 is_correct: option_row.is_correct,
//                 order: option_row.order as u32,
//             });
//         }
//     }

//     // Update question count on the chapter
//     let _ = sqlx::query!(
//         r#"
//         UPDATE chapters SET question_count = question_count + 1 WHERE id = $1
//         "#,
//         input.chapter_id as i32
//     )
//     .execute(&*pool)
//     .await;

//     let question_type = match question_row.question_type.as_str() {
//         "mcq" => QuestionType::MCQ,
//         "cq" => QuestionType::CQ,
//         _ => QuestionType::CQ,
//     };

//     Ok(Question {
//         id: question_row.id as u32,
//         question_text: question_row.question_text,
//         question_type,
//         chapter_id: question_row.chapter_id as u32,
//         class_id: question_row.class_id as u32,
//         subject_id: question_row.subject_id as u32,
//         difficulty: question_row.difficulty as u32,
//         answer_text: question_row.answer_text,
//         created_at: question_row.created_at.to_string(),
//         updated_at: question_row.updated_at.to_string(),
//         options,
//     })
// }

// #[cfg(feature = "ssr")]
// pub async fn update_question(
//     pool: std::sync::Arc<sqlx::PgPool>,
//     input: UpdateQuestionInput,
// ) -> Result<Question, crate::error::QuestionBankError> {
//     use anyhow::Context;
//     let question_type_str = input.question_type.as_str();

//     // Insert the question
//     let question_row = sqlx::query!(
//         r#"
//         UPDATE questions SET question_text = $1, question_type = $2, chapter_id = $3, class_id = $4, subject_id = $5, difficulty = $6, answer_text = $7, updated_at = NOW()
//         WHERE id = $8
//         RETURNING id, question_text, question_type, chapter_id, class_id, subject_id, difficulty, answer_text, created_at, updated_at
//         "#,
//         input.question_text,
//         question_type_str,
//         input.chapter_id as i32,
//         input.class_id as i32,
//         input.subject_id as i32,
//         input.difficulty as i32,
//         input.answer_text.as_deref(),
//         input.id as i32
//     )
//     .fetch_one(&*pool)
//     .await
//     .context("Failed to insert question")?;

//     let question_id = question_row.id;

//     // delete previous/old optios first
//     sqlx::query!(
//         r#"DELETE FROM question_options WHERE question_id = $1"#,
//         question_id
//     )
//     .execute(&*pool)
//     .await
//     .context("Failed to delete Options")?;

//     // Insert options for mcq questions
//     let mut options = Vec::new();
//     if question_type_str == "mcq" {
//         for option_input in &input.options {
//             let option_row = sqlx::query!(
//                 r#"
//                 INSERT INTO question_options (question_id, option_text, is_correct, "order")
//                 VALUES ($1, $2, $3, $4)
//                 RETURNING id, option_text, is_correct, "order"
//                 "#,
//                 question_id,
//                 option_input.option_text,
//                 option_input.is_correct,
//                 option_input.order as i32,
//             )
//             .fetch_one(&*pool)
//             .await
//             .context("Failed to insert question option")?;

//             options.push(QuestionOption {
//                 id: option_row.id as u32,
//                 option_text: option_row.option_text,
//                 is_correct: option_row.is_correct,
//                 order: option_row.order as u32,
//             });
//         }
//     }

//     let question_type = match question_row.question_type.as_str() {
//         "mcq" => QuestionType::MCQ,
//         "cq" => QuestionType::CQ,
//         _ => QuestionType::CQ,
//     };

//     Ok(Question {
//         id: question_row.id as u32,
//         question_text: question_row.question_text,
//         question_type,
//         chapter_id: question_row.chapter_id as u32,
//         class_id: question_row.class_id as u32,
//         subject_id: question_row.subject_id as u32,
//         difficulty: question_row.difficulty as u32,
//         answer_text: question_row.answer_text,
//         created_at: question_row.created_at.to_string(),
//         updated_at: question_row.updated_at.to_string(),
//         options,
//     })
// }
