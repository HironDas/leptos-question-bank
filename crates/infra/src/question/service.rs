use std::collections::HashMap;

use anyhow::Context;
use domain::question::{AddQuestionInput, Question, QuestionOption, UpdateQuestionInput};

use crate::common::{db, ServiceContext, ServiceError};
use crate::question::repository::{self, map_question};
use crate::question::repository::{list_question_options, list_questions};

#[derive(Clone)]
pub struct QuestionService {
    pub(crate) context: ServiceContext,
}

impl From<ServiceContext> for QuestionService {
    fn from(context: ServiceContext) -> Self {
        Self { context }
    }
}

impl QuestionService {
    pub async fn load_questions(&self, chapter_id: i32) -> Result<Vec<Question>, ServiceError> {
        let pool = &self.context.pool;

        let (questions, options) = tokio::try_join!(
            list_questions(pool, chapter_id),
            list_question_options(pool, chapter_id)
        )?;

        // let mut options_by_question: HashMap<i32, Vec<QuestionOption>> = HashMap::new();

        let mut options_by_question = options.into_iter().fold(
            HashMap::<i32, Vec<QuestionOption>>::new(),
            |mut map, option| {
                map.entry(option.question_id)
                    .or_default()
                    .push(QuestionOption {
                        id: option.id,
                        option_text: option.option_text,
                        is_correct: option.is_correct,
                        order: option.order,
                    });
                map
            },
        );

        let questions: Vec<Question> = questions
            .into_iter()
            .map(|q| {
                let o = options_by_question.remove(&q.id).unwrap();
                map_question(q, o)
            })
            .collect::<domain::Result<Vec<Question>>>()?;

        Ok(questions)
    }

    pub async fn find_quesiton(&self, id: i32) -> Result<Question, ServiceError> {
        let pool = &self.context.pool;

        let questions = repository::find_question_by_id(pool, id).await?;
        let options = repository::list_options_by_question_id(pool, questions.id)
            .await?
            .into_iter()
            .map(|o| QuestionOption {
                id: o.id,
                option_text: o.option_text,
                is_correct: o.is_correct,
                order: o.order,
            })
            .collect::<Vec<QuestionOption>>();

        let question = map_question(questions, options)?;
        Ok(question)
    }

    pub async fn add_question(&self, input: AddQuestionInput) -> Result<Question, ServiceError> {
        let mut tx = self
            .context
            .pool
            .begin()
            .await
            .context("Failed to connect postgres tx")?;

        let question = repository::insert_question(&mut tx, &input).await?;

        let options = repository::insert_options(&mut tx, question.id, &input.options).await?;
        repository::increment_chapter_question_count(&mut tx, input.chapter_id).await?;

        tx.commit().await.map_err(db)?;

        Ok(map_question(question, options)?)
    }

    pub async fn edit_question(
        &self,
        input: UpdateQuestionInput,
    ) -> Result<Question, ServiceError> {
        let mut tx = self.context.pool.begin().await.map_err(db)?;

        let question = repository::update_question(&mut tx, &input).await?;
        repository::delete_old_options(&mut tx, question.id).await?;
        let options = repository::insert_options(&mut tx, question.id, &input.options).await?;

        tx.commit().await.map_err(db)?;

        Ok(map_question(question, options)?)
    }
}
