use include_dir::{include_dir, Dir};
use sqlx::Executor;
use sqlx::{Connection, PgConnection};

static SEED_DIRECTORY: Dir = include_dir!("seeds");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_uri = std::env::var("DATABASE_URL")?;
    let mut conn = PgConnection::connect(&database_uri).await?;

    let mut transaction = conn.begin().await?;

    println!("Migrating database…");
    sqlx::migrate!("./migrations").run(&mut transaction).await?;

    println!("Seeding database…");

    for file in SEED_DIRECTORY.files() {
        let Some(content) = file.contents_utf8() else {
            anyhow::bail!("not a UTF-8 file: {:?}", file.path());
        };

        let result = transaction.execute(content).await?;
        println!(
            "Executed seed file: {:?} with {:?} rows affected",
            file.path(),
            result.rows_affected()
        );
    }

    transaction.commit().await?;

    Ok(())
}
