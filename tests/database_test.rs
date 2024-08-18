#![feature(async_closure)]

mod common;

#[cfg(all(test, feature = "test_database"))]
mod tests {
    use backend;
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

        assert_eq!(schema.id, "shop");
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
}
