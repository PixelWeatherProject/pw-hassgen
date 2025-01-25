use crate::hass::Entity;
use log::{debug, info, warn};
use sqlx::{query_file, PgConnection};
use std::io::Write;

#[allow(clippy::future_not_send)]
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

    for node_id in nodes {
        debug!("Processing node #{node_id}");

        entities.push(Entity::new_temperature(node_id, db_url));
        entities.push(Entity::new_humidity(node_id, db_url));

        if stats {
            debug!("Processing stats for node");

            entities.push(Entity::new_battery(node_id, db_url));
            entities.push(Entity::new_wifi_essid(node_id, db_url));
            entities.push(Entity::new_wifi_rssi(node_id, db_url));
        }
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
