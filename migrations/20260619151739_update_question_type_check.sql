-- Add migration script here
-- 1. Drop the old check constraint (replace 'your_table_name' with your actual table name)
-- Note: SQLx automatically names check constraints, usually 'table_column_check'
ALTER TABLE questions
DROP CONSTRAINT IF EXISTS questions_question_type_check;

-- 2. Update existing rows so they don't break the new constraint
UPDATE questions SET question_type = 'mcq' WHERE question_type = 'objective';
UPDATE questions SET question_type = 'cq' WHERE question_type = 'subjective';

-- 3. Add the new check constraint
ALTER TABLE questions
ADD CONSTRAINT your_table_name_question_type_check
CHECK (question_type IN ('mcq', 'cq'));
