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
    pub id: u32,
    pub option_text: String,
    pub is_correct: bool,
    pub order: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub question_text: String,
    pub question_type: QuestionType,
    pub chapter_id: u32,
    pub class_id: u32,
    pub subject_id: u32,
    pub difficulty: u32,
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
    pub order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct AddQuestionInput {
    #[validate(length(min = 1, message = "Question text cannot be empty"))]
    pub question_text: String,
    #[validate(length(min = 1, message = "Question type is required"))]
    pub question_type: String, // "mcq" or "cq"
    #[validate(range(min = 1, message = "Chapter is required"))]
    pub chapter_id: u32,
    #[validate(range(min = 1, message = "Class is required"))]
    pub class_id: u32,
    #[validate(range(min = 1, message = "Subject is required"))]
    pub subject_id: u32,
    #[validate(range(min = 0, max = 2, message = "Difficulty is required"))]
    pub difficulty: u32,
    #[serde(default)]
    pub answer_text: Option<String>,
    #[serde(default)]
    pub options: Vec<AddQuestionOptionInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct UpdateQuestionInput {
    #[validate(range(min = 1, message = "Question id is required"))]
    pub id: u32,
    #[validate(length(min = 1, message = "Question text cannot be empty"))]
    pub question_text: String,
    #[validate(length(min = 1, message = "Question type is required"))]
    pub question_type: String, // "mcq" or "cq"
    #[validate(range(min = 1, message = "Chapter is required"))]
    pub chapter_id: u32,
    #[validate(range(min = 1, message = "Class is required"))]
    pub class_id: u32,
    #[validate(range(min = 1, message = "Subject is required"))]
    pub subject_id: u32,
    #[validate(range(min = 0, max = 2, message = "Difficulty is required"))]
    pub difficulty: u32,
    #[serde(default)]
    pub answer_text: Option<String>,
    #[serde(default)]
    pub options: Vec<AddQuestionOptionInput>,
}
