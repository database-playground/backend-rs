use include_dir::include_dir;
use include_dir::Dir;
use sqlx::{Connection, Executor, Postgres};

static SEED_DIRECTORY: Dir = include_dir!("seeds");

pub async fn seed_connection(
    conn: &mut impl Connection<Database = Postgres>,
) -> anyhow::Result<()> {
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
