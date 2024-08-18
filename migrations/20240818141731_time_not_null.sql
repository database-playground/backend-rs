-- Add migration script here

ALTER TABLE dp_schemas ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE dp_schemas ALTER COLUMN updated_at SET NOT NULL;

ALTER TABLE dp_questions ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE dp_questions ALTER COLUMN updated_at SET NOT NULL;

ALTER TABLE dp_groups ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE dp_groups ALTER COLUMN updated_at SET NOT NULL;
