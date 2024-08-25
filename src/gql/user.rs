use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
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
#[graphql(complex)]
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

#[ComplexObject]
impl User {
    async fn group<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Group>> {
        tracing::debug!("Running GraphQL query 'group'");

        let Some(group_id) = self.group_id else {
            return Ok(None);
        };

        let pool = ctx.data::<db::Pool>()?;
        let group = db::get_group(pool, group_id).await?;

        Ok(Some(group.into()))
    }
}

#[derive(SimpleObject)]
pub struct Group {
    pub group_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<db::Group> for Group {
    fn from(group: db::Group) -> Self {
        Self {
            group_id: group.group_id,
            name: group.name,
            description: group.description,
            created_at: group.created_at,
            updated_at: group.updated_at,
        }
    }
}
