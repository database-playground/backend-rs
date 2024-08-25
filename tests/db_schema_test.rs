#![cfg(all(test, feature = "test_database"))]

#[sqlx::test(fixtures("schema"))]
async fn test_get_schema(pool: sqlx::PgPool) {
    let schema = backend::db::get_schema(&pool, "shop")
        .await
        .expect("failed to get schema");

    assert_eq!(schema.schema_id, "shop");
    assert_eq!(schema.picture, None);
    assert_eq!(schema.description, "The schema that is for a shop");

    println!("{schema:?}");
}

#[sqlx::test(fixtures("schema"))]
async fn test_get_schema_initial_sql(pool: sqlx::PgPool) {
    let initial_sql = backend::db::get_schema_initial_sql(&pool, "shop")
        .await
        .expect("failed to get schema initial sql");

    assert!(
        initial_sql.contains("CREATE TABLE products"),
        "initial sql does not contain CREATE TABLE products"
    );

    println!("{initial_sql}");
}
