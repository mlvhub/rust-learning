use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let row: Vec<(i32, String)> = sqlx::query_as("SELECT id, name FROM books")
        .fetch_all(&pool)
        .await?;

    println!("Got: {:?}", row);

    Ok(())
}
