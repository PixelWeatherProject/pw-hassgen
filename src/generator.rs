use log::{debug, info};
use sqlx::{query_file, PgConnection};
use std::io::Write;

pub async fn run(
    stats: bool,
    verify: bool,
    skip: &[u16],
    mut db: PgConnection,
    out: impl Write,
) -> anyhow::Result<()> {
    let mut nodes = get_node_ids(&mut db).await?;
    debug!("Got node IDs: {nodes:?}");

    debug!("Filtering nodes: {skip:?}");
    nodes.retain(|id| skip.contains(id));
    info!("Working with nodes: {nodes:?}");

    Ok(())
}

async fn get_node_ids(db: &mut PgConnection) -> anyhow::Result<Vec<u16>> {
    let results = query_file!("queries/get_node_ids.sql")
        .fetch_all(db)
        .await?;

    let ids: Vec<u16> = results
        .iter()
        .flat_map(|record| record.id.try_into())
        .collect();

    Ok(ids)
}
