#![cfg(all(test, feature = "test_database"))]
#![feature(assert_matches)]

use std::assert_matches::assert_matches;

use backend::db::{self, Cursor};
use sqlx::PgPool;

#[sqlx::test(fixtures("schema", "question"))]
async fn test_list_questions_default_cursor(pool: PgPool) {
    let questions = backend::db::list_questions(&pool, Cursor::default())
        .await
        .expect("failed to list questions");

    assert_eq!(questions.len(), 10, "default offset=0, limit=10");
    assert_eq!(questions[0].title, "Find a product in the shop");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_list_questions_offset_1_limit_5(pool: PgPool) {
    let questions = backend::db::list_questions(
        &pool,
        Cursor {
            offset: Some(1),
            limit: Some(5),
        },
    )
    .await
    .expect("failed to list questions");

    assert_eq!(questions.len(), 5, "offset=1, limit=5");
    assert_eq!(questions[0].title, "List all customers");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_list_questions_offset_5_limit_5(pool: PgPool) {
    let questions = backend::db::list_questions(
        &pool,
        Cursor {
            offset: Some(5),
            limit: Some(5),
        },
    )
    .await
    .expect("failed to list questions");

    assert_eq!(questions.len(), 5, "offset=5, limit=5");
    assert_eq!(questions[0].title, "Find a book by title");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_list_questions_offset_0_limit_5(pool: PgPool) {
    let questions = backend::db::list_questions(
        &pool,
        Cursor {
            offset: Some(0),
            limit: Some(5),
        },
    )
    .await
    .expect("failed to list questions");

    assert_eq!(questions.len(), 5, "offset=0, limit=5");
    assert_eq!(questions[0].title, "Find a product in the shop");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_list_questions_limit_0(pool: PgPool) {
    let questions = backend::db::list_questions(
        &pool,
        Cursor {
            limit: Some(0),
            ..Default::default()
        },
    )
    .await
    .expect("failed to list questions");

    assert_eq!(questions.len(), 0, "limit=0 should returns nothing");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_list_questions_limit_none(pool: PgPool) {
    let questions = backend::db::list_questions(
        &pool,
        Cursor {
            limit: None,
            ..Default::default()
        },
    )
    .await
    .expect("failed to list questions");

    assert_eq!(questions.len(), 10, "limit=None should returns 10");
    assert_eq!(questions[0].title, "Find a product in the shop");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question(pool: PgPool) {
    let question = backend::db::get_question(&pool, 1)
        .await
        .expect("failed to get question");

    assert_eq!(question.title, "Find a product in the shop");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_answer(pool: PgPool) {
    let question_answer = backend::db::get_question_answer(&pool, 1)
        .await
        .expect("failed to get question answer");

    assert_eq!(
        question_answer,
        "SELECT * FROM products WHERE product_name = 'Laptop';"
    );
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_solution_not_null(pool: PgPool) {
    let question_solution = backend::db::get_question_solution(&pool, 1)
        .await
        .expect("failed to get question solution");

    assert_matches!(
        question_solution,
        Some(url) if url == "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
    );
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_solution_null(pool: PgPool) {
    let question_solution = backend::db::get_question_solution(&pool, 2)
        .await
        .expect("failed to get question solution");

    assert_eq!(question_solution, None);
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_not_found(pool: PgPool) {
    let question = backend::db::get_question(&pool, 114514).await;

    assert_matches!(
        question,
        Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "114514",
        "question should not be found"
    );
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_answer_not_found(pool: PgPool) {
    let question_answer = backend::db::get_question_answer(&pool, 114514).await;

    assert_matches!(
        question_answer,
        Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "114514",
        "question answer should not be found"
    );
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_solution_not_found(pool: PgPool) {
    let question_solution = backend::db::get_question_solution(&pool, 114514).await;

    assert_matches!(
        question_solution,
        Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "114514",
        "question solution should not be found"
    );
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_deleted(pool: PgPool) {
    let question = backend::db::get_question(&pool, 20).await;

    assert_matches!(question, Err(db::Error::NotFound {
        entity: "question",
        id,
    }) if id == "20");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_answer_deleted(pool: PgPool) {
    let question_answer = backend::db::get_question_answer(&pool, 20).await;

    assert_matches!(question_answer, Err(db::Error::NotFound {
        entity: "question",
        id,
    }) if id == "20");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_solution_deleted(pool: PgPool) {
    let question_solution = backend::db::get_question_solution(&pool, 20).await;

    assert_matches!(question_solution, Err(db::Error::NotFound {
        entity: "question",
        id,
    }) if id == "20");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_schema_initial_sql(pool: PgPool) {
    let initial_sql = backend::db::get_question_schema_initial_sql(&pool, 1)
        .await
        .expect("failed to get question schema initial sql");

    assert!(
        initial_sql.contains("CREATE TABLE products"),
        "initial sql does not contain CREATE TABLE products"
    );

    println!("{initial_sql}");
}

#[sqlx::test(fixtures("schema", "question"))]
async fn test_get_question_schema_initial_sql_not_found(pool: PgPool) {
    let schema = backend::db::get_question_schema_initial_sql(&pool, 114514).await;

    assert_matches!(
        schema,
        Err(db::Error::NotFound {
            entity: "question",
            id,
        }) if id == "114514",
        "question schema should not be found"
    );
}
