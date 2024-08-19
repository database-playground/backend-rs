#![feature(async_closure, assert_matches)]

mod common;

#[cfg(all(test, feature = "test_database"))]
mod tests {
    use std::assert_matches::assert_matches;

    use backend::{self, db};
    use tokio;

    #[tokio::test]
    async fn test_get_schema() {
        let schema = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::get_schema(c, "shop").await?;
            Ok(result)
        })
        .await
        .expect("failed to get schema");

        assert_eq!(schema.schema_id, "shop");
        assert_eq!(schema.picture, None);
        assert_eq!(schema.description, "The schema that is for a shop");

        println!("{schema:?}");
    }

    #[tokio::test]
    async fn test_get_schema_initial_sql() {
        let initial_sql = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::get_schema_initial_sql(c, "shop").await?;
            Ok(result)
        })
        .await
        .expect("failed to get schema initial sql");

        assert!(
            initial_sql.contains("CREATE TABLE products"),
            "initial sql does not contain CREATE TABLE products"
        );

        println!("{initial_sql}");
    }

    #[tokio::test]
    async fn test_get_schema_deleted() {
        let schema = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::get_schema(c, "deleted_schema").await?;
            Ok(result)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(
            schema,
            Err(db::Error::NotFound {
                entity,
                id,
            }) if entity == "schema" && id == "deleted_schema",
            "schema should not be found"
        );
    }

    #[tokio::test]
    async fn test_get_schema_initial_sql_deleted() {
        let initial_sql = crate::common::run_onetime(async |c| {
            backend::db::seeder::seed_connection(c).await?;

            let result = backend::db::get_schema_initial_sql(c, "deleted_schema").await?;
            Ok(result)
        })
        .await
        .map_err(|e| e.downcast::<db::Error>().expect("failed to downcast error"));

        assert_matches!(
            initial_sql,
            Err(db::Error::NotFound {
                entity,
                id,
            }) if entity == "schema" && id == "deleted_schema",
            "schema initial sql should not be found"
        );
    }
}
