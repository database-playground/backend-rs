use super::{Error, Executor};

#[derive(Debug, Clone, Copy, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "dp_attempt_status", rename_all = "lowercase")]
pub enum AttemptStatus {
    Pending,
    Passed,
    Failed,
}

#[tracing::instrument(skip(conn))]
pub async fn create_attempt_event(
    conn: impl Executor<'_>,
    user_id: &str,
    question_id: i64,
    query: &str,
    status: AttemptStatus,
) -> Result<i64, Error> {
    tracing::debug!("Creating attempt event in database");

    let event = sqlx::query!(
        r#"
        INSERT INTO dp_attempt_events (user_id, question_id, query, status)
        VALUES ($1, $2, $3, $4)
        RETURNING (attempt_event_id)
        "#,
        user_id,
        question_id,
        query,
        status as AttemptStatus
    )
    .fetch_one(conn)
    .await?;

    Ok(event.attempt_event_id)
}

#[tracing::instrument(skip(conn))]
pub async fn mark_attempt_event(
    conn: impl Executor<'_>,
    event_id: i64,
    status: AttemptStatus,
) -> Result<(), Error> {
    tracing::debug!("Marking attempt event in database");

    sqlx::query!(
        r#"
        UPDATE dp_attempt_events
        SET status = $1
        WHERE attempt_event_id = $2
        "#,
        status as AttemptStatus,
        event_id,
    )
    .execute(conn)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(conn))]
pub async fn create_solution_event(
    conn: impl Executor<'_>,
    user_id: &str,
    question_id: i64,
) -> Result<i64, Error> {
    tracing::debug!("Creating solution event in database");

    let event = sqlx::query!(
        r#"
        INSERT INTO dp_solution_events (user_id, question_id)
        VALUES ($1, $2)
        RETURNING (solution_event_id)
        "#,
        user_id,
        question_id,
    )
    .fetch_one(conn)
    .await?;

    Ok(event.solution_event_id)
}
