use crate::hass::Entity;
use log::{debug, info, warn};
use sqlx::{query_file, PgConnection};
use std::io::Write;

pub async fn run(
    stats: bool,
    verify: bool,
    skip: &[u16],
    mut db: PgConnection,
    db_url: &str,
    out: impl Write,
) -> anyhow::Result<()> {
    let mut nodes = get_node_ids(&mut db).await?;
    debug!("Got node IDs: {nodes:?}");

    debug!("Filtering nodes: {skip:?}");
    nodes.retain(|id| !skip.contains(id));
    info!("Working with nodes: {nodes:?}");

    let mut entities = Vec::new();

    // Temperature and Humidity are required, so we can do them here
    for node_id in nodes {
        debug!("Processing node #{node_id}");

        let temperature = Entity::new_temperature(node_id, db_url);
        let humidity = Entity::new_humidity(node_id, db_url);

        entities.push(temperature);
        entities.push(humidity);
    }

    if verify {
        verify_queries(&entities, &mut db).await?;
        info!("Query verification OK");
    } else {
        info!("Skipping query verification");
    }

    serde_yaml::to_writer(out, &entities)?;

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

async fn verify_queries(entities: &[Entity], db: &mut PgConnection) -> anyhow::Result<()> {
    for entity in entities {
        let query = entity.query();
        let rows = sqlx::query(query).fetch_all(&mut *db).await?;

        if rows.len() > 1 {
            warn!("Entity \"{}\"'s query returns multiple rows", entity.name());
        }
    }

    Ok(())
}
