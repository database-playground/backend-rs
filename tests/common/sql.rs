use sqlx::Connection;
use sqlx::Executor;
use sqlx::PgConnection;

pub async fn run_onetime<T>(
    f: impl async FnOnce(&mut PgConnection) -> Result<T, anyhow::Error>,
) -> Result<T, anyhow::Error> {
    const PRIMARY_DATABASE_URL: &str = env!("DATABASE_URL");

    let mut primary_conn = PgConnection::connect(PRIMARY_DATABASE_URL).await?;

    // create a one-time database
    let database_name = format!("dp_test_{}", uuid::Uuid::new_v4().as_simple());
    println!("Creating one-time database: {}", database_name);

    primary_conn
        .execute(format!("CREATE DATABASE {database_name}").as_str())
        .await
        .expect("failed to create onetime database");

    // create a connection to the one-time database
    let result = {
        let database_uri_without_db = match PRIMARY_DATABASE_URL.rfind('/') {
            Some(idx) => &PRIMARY_DATABASE_URL[..idx],
            None => PRIMARY_DATABASE_URL,
        };
        let temporary_connection_url = format!("{database_uri_without_db}/{database_name}");

        create_temp_connection_then_run_onetime(&temporary_connection_url, f).await
    };

    // clear up the one-time database
    println!("Dropping one-time database: {}", database_name);
    primary_conn
        .execute(format!("DROP DATABASE {database_name}").as_str())
        .await
        .expect("failed to drop onetime database");

    result
}

async fn create_temp_connection_then_run_onetime<T>(
    database_url: &str,
    f: impl async FnOnce(&mut PgConnection) -> Result<T, anyhow::Error>,
) -> Result<T, anyhow::Error> {
    let mut c = PgConnection::connect(database_url).await?;
    let result = f(&mut c).await;
    c.close().await?;

    result
}
