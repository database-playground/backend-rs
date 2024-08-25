-- Shop Schema
INSERT INTO dp_schemas (schema_id, description, initial_sql)
VALUES (
    'shop',
    'The schema that is for a shop',
    'CREATE TABLE products (
        product_id INT PRIMARY KEY,
        product_name VARCHAR(100),
        price DECIMAL(10, 2),
        stock INT
    );

    CREATE TABLE customers (
        customer_id INT PRIMARY KEY,
        customer_name VARCHAR(100),
        email VARCHAR(100)
    );

    CREATE TABLE orders (
        order_id INT PRIMARY KEY,
        customer_id INT,
        order_date DATE,
        total DECIMAL(10, 2),
        FOREIGN KEY (customer_id) REFERENCES customers(customer_id)
    );

    INSERT INTO products (product_id, product_name, price, stock) VALUES
    (1, ''Laptop'', 999.99, 10),
    (2, ''Mouse'', 19.99, 100);

    INSERT INTO customers (customer_id, customer_name, email) VALUES
    (1, ''Alice'', ''alice@example.com''),
    (2, ''Bob'', ''bob@example.com'');

    INSERT INTO orders (order_id, customer_id, order_date, total) VALUES
    (1, 1, ''2024-01-01'', 1019.98);'
);
-- Library Schema
INSERT INTO dp_schemas (schema_id, description, initial_sql)
VALUES (
    'library',
    'The schema that is for a library',
    'CREATE TABLE books (
        book_id INT PRIMARY KEY,
        title VARCHAR(200),
        author VARCHAR(100),
        genre VARCHAR(50),
        published_date DATE
    );

    CREATE TABLE members (
        member_id INT PRIMARY KEY,
        member_name VARCHAR(100),
        membership_date DATE
    );

    CREATE TABLE borrowings (
        borrowing_id INT PRIMARY KEY,
        book_id INT,
        member_id INT,
        borrowing_date DATE,
        return_date DATE,
        FOREIGN KEY (book_id) REFERENCES books(book_id),
        FOREIGN KEY (member_id) REFERENCES members(member_id)
    );

    INSERT INTO books (book_id, title, author, genre, published_date) VALUES
    (1, ''1984'', ''George Orwell'', ''Dystopian'', ''1949-06-08''),
    (2, ''To Kill a Mockingbird'', ''Harper Lee'', ''Fiction'', ''1960-07-11'');

    INSERT INTO members (member_id, member_name, membership_date) VALUES
    (1, ''John Doe'', ''2023-01-15''),
    (2, ''Jane Smith'', ''2022-12-20'');

    INSERT INTO borrowings (borrowing_id, book_id, member_id, borrowing_date, return_date) VALUES
    (1, 1, 1, ''2024-06-01'', ''2024-06-15'');'
);
-- School Schema
INSERT INTO dp_schemas (schema_id, description, initial_sql)
VALUES (
    'school',
    'The schema that is for a school',
    'CREATE TABLE students (
        student_id INT PRIMARY KEY,
        student_name VARCHAR(100),
        date_of_birth DATE,
        grade INT
    );

    CREATE TABLE teachers (
        teacher_id INT PRIMARY KEY,
        teacher_name VARCHAR(100),
        subject VARCHAR(50)
    );

    CREATE TABLE classes (
        class_id INT PRIMARY KEY,
        class_name VARCHAR(50),
        teacher_id INT,
        schedule_time TIME,
        FOREIGN KEY (teacher_id) REFERENCES teachers(teacher_id)
    );

    CREATE TABLE enrollments (
        enrollment_id INT PRIMARY KEY,
        student_id INT,
        class_id INT,
        enrollment_date DATE,
        FOREIGN KEY (student_id) REFERENCES students(student_id),
        FOREIGN KEY (class_id) REFERENCES classes(class_id)
    );

    INSERT INTO students (student_id, student_name, date_of_birth, grade) VALUES
    (1, ''Emily'', ''2010-05-12'', 5),
    (2, ''Daniel'', ''2011-07-23'', 4);

    INSERT INTO teachers (teacher_id, teacher_name, subject) VALUES
    (1, ''Mr. Brown'', ''Mathematics''),
    (2, ''Ms. Green'', ''Science'');

    INSERT INTO classes (class_id, class_name, teacher_id, schedule_time) VALUES
    (1, ''Math 101'', 1, ''09:00:00''),
    (2, ''Science 101'', 2, ''10:00:00'');

    INSERT INTO enrollments (enrollment_id, student_id, class_id, enrollment_date) VALUES
    (1, 1, 1, ''2024-07-01''),
    (2, 2, 2, ''2024-07-01'');'
);
INSERT INTO dp_schemas (schema_id, description, initial_sql, deleted_at)
VALUES (
    'deleted_schema',
    '',
    '',
    '2024-08-20 02:11:00'
);
