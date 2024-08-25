INSERT INTO dp_questions (
    schema_id,
    type,
    difficulty,
    title,
    description,
    answer,
    solution_video
) VALUES (
    'shop',
    '條件查詢',
    'easy',
    'Find a product in the shop',
    'Write a SQL query to find the ''Laptop'' product in the shop schema.',
    'SELECT * FROM products WHERE product_name = ''Laptop'';',
    'https://www.youtube.com/watch?v=dQw4w9WgXcQ'
);

INSERT INTO dp_questions (
    schema_id,
    type,
    difficulty,
    title,
    description,
    answer
)
VALUES
(
    'shop',
    '條件查詢',
    'easy',
    'List all customers',
    'Write a SQL query to list all customers in the shop schema.',
    'SELECT * FROM customers;'
),
(
    'shop',
    '條件查詢',
    'medium',
    'Find orders by a specific customer',
    'Write a SQL query to find all orders placed by the customer with ID 1.',
    'SELECT * FROM orders WHERE customer_id = 1;'
),
(
    'shop',
    '群組應用',
    'medium',
    'Find products below a certain stock level',
    'Write a SQL query to find all products with a stock level less than 20.',
    'SELECT * FROM products WHERE stock < 20;'
),
(
    'library',
    '群組應用',
    'easy',
    'List all books',
    'Write a SQL query to list all books in the library schema.',
    'SELECT * FROM books;'
),
(
    'library',
    '群組應用',
    'medium',
    'Find a book by title',
    'Write a SQL query to find the book ''1984'' by George Orwell.',
    'SELECT * FROM books WHERE title = ''1984'';'
),
(
    'library',
    '子查詢應用',
    'easy',
    'List all members',
    'Write a SQL query to list all members in the library schema.',
    'SELECT * FROM members;'
),
(
    'library',
    '子查詢應用',
    'medium',
    'Find borrowings by a specific member',
    'Write a SQL query to find all borrowings by the member with ID 1.',
    'SELECT * FROM borrowings WHERE member_id = 1;'
),
(
    'library',
    '子查詢應用',
    'hard',
    'Find overdue borrowings',
    'Write a SQL query to find all borrowings \
    where the return date is past due (today''s date is ''2024-07-16'').',
    'SELECT * FROM borrowings WHERE return_date < ''2024-07-16'';'
),
(
    'school',
    '子查詢+群組綜合應用',
    'easy',
    'List all students',
    'Write a SQL query to list all students in the school schema.',
    'SELECT * FROM students;'
),
(
    'school',
    '子查詢+群組綜合應用',
    'medium',
    'Find students in a specific grade',
    'Write a SQL query to find all students in grade 5.',
    'SELECT * FROM students WHERE grade = 5;'
),
(
    'school',
    '子查詢+群組綜合應用',
    'easy',
    'List all teachers',
    'Write a SQL query to list all teachers in the school schema.',
    'SELECT * FROM teachers;'
),
(
    'school',
    '進階外部查詢',
    'medium',
    'Find classes taught by a specific teacher',
    'Write a SQL query to find all classes taught by the teacher with ID 1.',
    'SELECT * FROM classes WHERE teacher_id = 1;'
),
(
    'school',
    '進階外部查詢',
    'easy',
    'List all classes',
    'Write a SQL query to list all classes in the school schema.',
    'SELECT * FROM classes;'
),
(
    'school',
    '進階外部查詢',
    'medium',
    'Find enrollments by a specific student',
    'Write a SQL query to find all enrollments for the student with ID 1.',
    'SELECT * FROM enrollments WHERE student_id = 1;'
),
(
    'school',
    '聯集應用',
    'hard',
    'Find students enrolled in a specific class',
    'Write a SQL query to find all students enrolled in the class \
    with ID 1.',
    'SELECT s.student_id, s.student_name FROM students s \
    JOIN enrollments e ON s.student_id = e.student_id WHERE e.class_id = 1;'
),
(
    'shop',
    '聯集應用',
    'hard',
    'Calculate total sales',
    'Write a SQL query to calculate the total sales in the shop.',
    'SELECT SUM(total) AS total_sales FROM orders;'
),
(
    'library',
    '聯集應用',
    'medium',
    'Find books by genre',
    'Write a SQL query to find all books in the genre ''Fiction''.',
    'SELECT * FROM books WHERE genre = ''Fiction'';'
),
(
    'school',
    '進階Exists指令應用',
    'hard',
    'Find students older than a specific age',
    'Write a SQL query to find all students older than 10 \
    years old (assuming today''s date is ''2024-07-16'').',
    'SELECT * FROM students WHERE date_of_birth < ''2014-07-16'';'
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
