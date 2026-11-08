use domain::question::Question;
use domain::Result;
use sqlx::types::uuid::timestamp::context;

use crate::common::ServiceContext;

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
    pub async fn load_questions(&self, _chapter_id: i32) -> Result<Vec<Question>> {
        todo!()
    }

    pub async fn get_quesiton(&self, q_id: i32) -> Result<Question> {
        todo!()
    }

    pub async fn add_question(&self) -> Result<()> {
        todo!()
    }

    pub async fn update_question(&self) -> Result<()> {
        todo!()
    }
}
