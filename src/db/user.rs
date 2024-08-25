use ecow::eco_format;

use super::{Acquire, Error, Executor};

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub group_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[tracing::instrument(skip(conn))]
pub async fn get_or_initialize_user(conn: impl Acquire<'_>, user_id: &str) -> Result<User, Error> {
    tracing::debug!("Getting user from database");

    let mut conn = conn.acquire().await?;

    let user_info = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, group_id, created_at, updated_at, deleted_at
        FROM dp_users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_optional(&mut *conn)
    .await?;

    if let Some(user_info) = user_info {
        if user_info.deleted_at.is_some() {
            return Err(Error::UserDeleted);
        }

        return Ok(user_info);
    }

    tracing::debug!("User not found, initializing user");

    let created_user_info = sqlx::query_as!(
        User,
        r#"
        INSERT INTO dp_users (user_id)
        VALUES ($1)
        RETURNING user_id, group_id, created_at, updated_at, deleted_at
        "#,
        user_id,
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(created_user_info)
}

/// Mark the user as deleted.
///
/// You should also remove this user from the authentication service.
pub async fn delete_user(conn: impl Executor<'_>, user_id: &str) -> Result<(), Error> {
    tracing::debug!("Deleting user");

    let affected_rows = sqlx::query!(
        r#"
        UPDATE dp_users
        SET deleted_at = now()
        WHERE user_id = $1 AND deleted_at IS NULL
        "#,
        user_id,
    )
    .execute(conn)
    .await?
    .rows_affected();

    if affected_rows == 0 {
        return Err(Error::NotFound {
            entity: "user",
            id: eco_format!("{user_id}"),
        });
    }

    Ok(())
}

pub struct GroupCreateParameter<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

#[tracing::instrument(skip(conn))]
pub async fn create_group(
    conn: impl Executor<'_>,
    GroupCreateParameter { name, description }: GroupCreateParameter<'_>,
) -> Result<i64, Error> {
    tracing::debug!("Creating group");

    let group_id = sqlx::query!(
        r#"
        INSERT INTO dp_groups (name, description)
        VALUES ($1, $2)
        RETURNING group_id
        "#,
        name,
        description.unwrap_or(""),
    )
    .fetch_one(conn)
    .await?
    .group_id;

    Ok(group_id)
}

#[derive(Debug, Clone)]
pub struct Group {
    pub group_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(skip(conn))]
pub async fn get_group(conn: impl Executor<'_>, group_id: i64) -> Result<Group, Error> {
    tracing::debug!("Getting group from database");

    let group = sqlx::query_as!(
        Group,
        r#"
        SELECT group_id, name, description, created_at, updated_at
        FROM dp_groups
        WHERE group_id = $1 AND deleted_at IS NULL
        "#,
        group_id,
    )
    .fetch_one(conn)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "group",
            id: eco_format!("{group_id}"),
        },
        _ => Error::DatabaseError(e),
    })?;

    Ok(group)
}
