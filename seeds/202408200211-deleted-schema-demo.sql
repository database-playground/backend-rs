INSERT INTO dp_schemas (schema_id, description, initial_sql, deleted_at)
VALUES (
    'deleted_schema',
    '',
    '',
    '2024-08-20 02:11:00'
);

INSERT INTO dp_questions (
    schema_id,
    type,
    difficulty,
    title,
    description,
    answer,
    deleted_at
) VALUES (
    'shop',
    '條件查詢',
    'easy',
    'Deleted question',
    'Write a SQL query to find the ''Laptop'' product in the shop schema.',
    'SELECT * FROM products WHERE product_name = ''Laptop'';',
    '2024-08-20 02:11:00'
);
