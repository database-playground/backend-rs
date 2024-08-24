//! Question-related database operations.

use chrono::{DateTime, Utc};

use super::{cursor::Cursor, Error, Executor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub question_id: i64,
    pub schema_id: Option<String>,
    pub question_type: String,
    pub difficulty: Difficulty,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "dp_difficulty", rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[tracing::instrument(skip(conn))]
pub async fn list_questions(
    conn: impl Executor<'_>,
    cursor: Cursor,
) -> Result<Vec<Question>, Error> {
    tracing::debug!("Listing questions from database");

    sqlx::query_as!(
        Question,
        r#"
        SELECT question_id, schema_id, type AS question_type, difficulty AS "difficulty: _", title, description, created_at, updated_at
        FROM dp_questions
        WHERE deleted_at IS NULL
        ORDER BY question_id
        LIMIT $1 OFFSET $2;
        "#,
        cursor.get_limit(),
        cursor.get_offset()
    )
    .fetch_all(conn)
    .await
    .map_err(Error::DatabaseError)
}

#[tracing::instrument(skip(conn))]
pub async fn get_question(conn: impl Executor<'_>, question_id: i64) -> Result<Question, Error> {
    tracing::debug!("Getting question from database");

    if question_id < 0 {
        return Err(Error::NotPositiveID);
    }

    sqlx::query_as!(
        Question,
        r#"
        SELECT question_id, schema_id, type AS question_type, difficulty AS "difficulty: _", title, description, created_at, updated_at
        FROM dp_questions
        WHERE question_id = $1 AND deleted_at IS NULL
        "#,
        question_id
    )
    .fetch_one(conn)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "question",
            id: ecow::eco_format!("{question_id}"),
        },
        e => Error::DatabaseError(e),
    })
}

#[tracing::instrument(skip(conn))]
pub async fn get_question_answer(
    conn: impl Executor<'_>,
    question_id: i64,
) -> Result<String, Error> {
    tracing::debug!("Getting question answer from database");

    if question_id < 0 {
        return Err(Error::NotPositiveID);
    }

    sqlx::query!(
        r#"
        SELECT answer
        FROM dp_questions
        WHERE question_id = $1 AND deleted_at IS NULL
        "#,
        question_id
    )
    .fetch_one(conn)
    .await
    .map(|record| record.answer)
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "question",
            id: ecow::eco_format!("{question_id}"),
        },
        e => Error::DatabaseError(e),
    })
}

#[tracing::instrument(skip(conn))]
pub async fn get_question_solution(
    conn: impl Executor<'_>,
    question_id: i64,
) -> Result<Option<String>, Error> {
    tracing::debug!("Getting question solution from database");

    if question_id < 0 {
        return Err(Error::NotPositiveID);
    }

    sqlx::query!(
        r#"
        SELECT solution_video
        FROM dp_questions
        WHERE question_id = $1 AND deleted_at IS NULL
        "#,
        question_id
    )
    .fetch_one(conn)
    .await
    .map(|record| record.solution_video)
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "question",
            id: ecow::eco_format!("{question_id}"),
        },
        e => Error::DatabaseError(e),
    })
}

pub async fn get_question_schema_initial_sql(
    conn: impl Executor<'_>,
    question_id: i64,
) -> Result<String, Error> {
    tracing::debug!("Getting question schema initial SQL from database");

    if question_id < 0 {
        return Err(Error::NotPositiveID);
    }

    sqlx::query!(
        r#"
        SELECT initial_sql
        FROM dp_questions
        JOIN dp_schemas USING (schema_id)
        WHERE question_id = $1 AND dp_questions.deleted_at IS NULL
        "#,
        question_id
    )
    .fetch_one(conn)
    .await
    .map(|record| record.initial_sql)
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "question",
            id: ecow::eco_format!("{question_id}"),
        },
        e => Error::DatabaseError(e),
    })
}
