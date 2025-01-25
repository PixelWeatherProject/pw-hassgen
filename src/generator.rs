use sqlx::PgConnection;
use std::io::Write;

pub async fn run(db: PgConnection, out: impl Write) -> anyhow::Result<()> {
    // ...

    Ok(())
}
