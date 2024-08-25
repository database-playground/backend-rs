use async_graphql::{Context, Object, Result, SimpleObject};
use ecow::EcoString;

use super::error::Error;
use crate::{db, gql::error::ErrorCode};

use super::auth::ContextAuthExt;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<User> {
        tracing::debug!("Running GraphQL query 'user'");

        let Some(sub) = ctx.sub() else {
            return Err(Error {
                code: ErrorCode::Unauthorized,
                title: EcoString::inline("Unauthorized"),
                details: "You must login to access this API.".into(),
                error: None,
            }
            .to_gql_error());
        };
        let pool = ctx.data::<db::Pool>()?;

        let user = db::get_or_initialize_user(pool, sub).await?;
        Ok(user.into())
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct User {
    pub user_id: String,
    pub group_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<db::User> for User {
    fn from(user: db::User) -> Self {
        Self {
            user_id: user.user_id,
            group_id: user.group_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
