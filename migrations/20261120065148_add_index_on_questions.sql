CREATE INDEX IF NOT EXISTS idx_questions_chapter_type_id
ON questions (chapter_id, question_type DESC, id ASC);

CREATE INDEX IF NOT EXISTS idx_question_options_question_order
ON question_options (question_id, "order");
