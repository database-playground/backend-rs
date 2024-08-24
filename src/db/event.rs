use sqlx::{Executor, Postgres};

use super::Error;

#[derive(Debug, Clone, Copy, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "dp_attempt_status", rename_all = "lowercase")]
pub enum AttemptStatus {
    Pending,
    Passed,
    Failed,
}

pub async fn create_attempt_event(
    conn: impl Executor<'_, Database = Postgres>,
    user_id: &str,
    question_id: i64,
    query: &str,
    status: AttemptStatus,
) -> Result<(), Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO dp_attempt_events (user_id, question_id, query, status)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        question_id,
        query,
        status as AttemptStatus
    );
    conn.execute(query).await?;

    Ok(())
}

pub async fn create_solution_event(
    conn: impl Executor<'_, Database = Postgres>,
    user_id: &str,
    question_id: i64,
) -> Result<(), Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO dp_solution_events (user_id, question_id)
        VALUES ($1, $2)
        "#,
        user_id,
        question_id,
    );
    conn.execute(query).await?;

    Ok(())
}
