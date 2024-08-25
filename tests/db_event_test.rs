#![cfg(all(test, feature = "test_database"))]

use backend::db::AttemptStatus;
use sqlx::PgPool;

#[sqlx::test(fixtures("group", "user", "schema", "question"))]
async fn test_create_attempt_event(pool: PgPool) {
    let event_id = backend::db::create_attempt_event(
        &pool,
        "usergeneric0",
        1,
        "SELECT 1;",
        backend::db::AttemptStatus::Pending,
    )
    .await
    .expect("failed to create attempt event");

    let attempt = sqlx::query!(
        r#"SELECT user_id, question_id, query, status AS "status: AttemptStatus" FROM dp_attempt_events WHERE attempt_event_id = $1;"#,
        event_id
    ).fetch_one(&pool).await.expect("failed to fetch attempt event");
    assert_eq!(attempt.user_id, "usergeneric0");
    assert_eq!(attempt.question_id, 1);
    assert_eq!(attempt.query, "SELECT 1;");
    assert_eq!(attempt.status, AttemptStatus::Pending);
}

#[sqlx::test(fixtures("group", "user", "schema", "question"))]
async fn test_mark_attempt_event(pool: PgPool) {
    let event_id = backend::db::create_attempt_event(
        &pool,
        "usergeneric0",
        1,
        "SELECT 1;",
        backend::db::AttemptStatus::Pending,
    )
    .await
    .expect("failed to create attempt event");

    backend::db::mark_attempt_event(&pool, event_id, backend::db::AttemptStatus::Passed)
        .await
        .expect("failed to mark attempt event");

    let attempt = sqlx::query!(
        r#"SELECT user_id, question_id, query, status AS "status: AttemptStatus" FROM dp_attempt_events WHERE attempt_event_id = $1;"#,
        event_id
    ).fetch_one(&pool).await.expect("failed to fetch attempt event");
    assert_eq!(attempt.user_id, "usergeneric0");
    assert_eq!(attempt.question_id, 1);
    assert_eq!(attempt.query, "SELECT 1;");
    assert_eq!(attempt.status, AttemptStatus::Passed);
}

#[sqlx::test(fixtures("group", "user", "schema", "question"))]
async fn test_create_solution_event(pool: PgPool) {
    let event_id = backend::db::create_solution_event(&pool, "usergeneric0", 1)
        .await
        .expect("failed to create solution event");

    let solution = sqlx::query!(
        r#"SELECT user_id, question_id FROM dp_solution_events WHERE solution_event_id = $1;"#,
        event_id
    )
    .fetch_one(&pool)
    .await
    .expect("failed to fetch solution event");
    assert_eq!(solution.user_id, "usergeneric0");
    assert_eq!(solution.question_id, 1);
}
