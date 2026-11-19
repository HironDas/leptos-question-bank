use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionType {
    #[serde(rename = "mcq")]
    MCQ,
    #[serde(rename = "cq")]
    CQ,
}

impl std::fmt::Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionType::MCQ => write!(f, "MCQ"),
            QuestionType::CQ => write!(f, "CQ"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: i32,
    pub option_text: String,
    pub is_correct: bool,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub question_text: String,
    pub question_type: QuestionType,
    pub chapter_id: i32,
    pub class_id: i32,
    pub subject_id: i32,
    pub difficulty: i32,
    pub answer_text: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub options: Vec<QuestionOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct AddQuestionOptionInput {
    #[validate(length(min = 1, message = "Option text cannot be empty"))]
    pub option_text: String,
    pub is_correct: bool,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct AddQuestionInput {
    #[validate(length(min = 1, message = "Question text cannot be empty"))]
    pub question_text: String,
    #[validate(length(min = 1, message = "Question type is required"))]
    pub question_type: String, // "mcq" or "cq"
    #[validate(range(min = 1, message = "Chapter is required"))]
    pub chapter_id: i32,
    #[validate(range(min = 1, message = "Class is required"))]
    pub class_id: i32,
    #[validate(range(min = 1, message = "Subject is required"))]
    pub subject_id: i32,
    #[validate(range(min = 0, max = 2, message = "Difficulty is required"))]
    pub difficulty: i32,
    #[serde(default)]
    pub answer_text: Option<String>,
    #[serde(default)]
    pub options: Vec<AddQuestionOptionInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct UpdateQuestionInput {
    #[validate(range(min = 1, message = "Question id is required"))]
    pub id: i32,
    #[validate(length(min = 1, message = "Question text cannot be empty"))]
    pub question_text: String,
    #[validate(length(min = 1, message = "Question type is required"))]
    pub question_type: String, // "mcq" or "cq"
    #[validate(range(min = 1, message = "Chapter is required"))]
    pub chapter_id: i32,
    #[validate(range(min = 1, message = "Class is required"))]
    pub class_id: i32,
    #[validate(range(min = 1, message = "Subject is required"))]
    pub subject_id: i32,
    #[validate(range(min = 0, max = 2, message = "Difficulty is required"))]
    pub difficulty: i32,
    #[serde(default)]
    pub answer_text: Option<String>,
    #[serde(default)]
    pub options: Vec<AddQuestionOptionInput>,
}
