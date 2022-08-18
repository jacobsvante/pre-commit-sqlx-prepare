use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sqlx::query!(r#"SELECT id FROM todos"#);
    Ok(())
}
