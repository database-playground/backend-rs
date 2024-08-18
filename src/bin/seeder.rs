#[tokio::main]
#[cfg(feature = "seeder")]
async fn main() -> anyhow::Result<()> {
    use sqlx::{Connection, PgConnection};

    let database_uri = std::env::var("DATABASE_URL")?;
    let mut conn = PgConnection::connect(&database_uri).await?;
    backend::db::seeder::seed_connection(&mut conn).await?;
    Ok(())
}

#[cfg(not(feature = "seeder"))]
fn main() {
    println!("This binary is not available without the `seeder` feature.");
    println!("Run `cargo run --bin seeder --features seeder` instead.");
}
