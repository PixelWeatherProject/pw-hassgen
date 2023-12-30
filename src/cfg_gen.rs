use crate::hass_cfg::{HassConfig, SensorEntry, SqlIntegrationConfig};
use sqlx::{PgConnection, Row};
use std::{error::Error, process::exit};

const ALWAYS_SUPPORTED_PROPS: &[(&str, &str, &str, &str)] = &[
    /* Property name, DB column, device class, unit */
    /* Device classes: https://www.home-assistant.io/integrations/number/ */
    ("Temperature", "temperature", "temperature", "°C"),
    ("Humidity", "humidity", "humidity", "%"),
];

const SENSOR_STATS: &[(&str, &str, Option<&str>, Option<&str>)] = &[
    ("Battery Voltage", "battery", Some("voltage"), Some("V")),
    ("WiFi SSID", "wifi_ssid", None, None),
    (
        "WiFi RSSI",
        "wifi_rssi",
        Some("signal_strength"),
        Some("dBm"),
    ),
];

const SENSOR_PROPS: &[(&str, &str, &str, &str)] = &[(
    "Air Pressure",
    "air_pressure",
    "atmospheric_pressure",
    "hPa",
)];

pub async fn generate_config(
    db_url: &str,
    mut db: PgConnection,
    include_stats: bool,
    verify: bool,
) -> Result<HassConfig, Box<dyn Error>> {
    let mut sensors = Vec::new();
    let device_ids = sqlx::query!("SELECT id FROM devices")
        .fetch_all(&mut db)
        .await?
        .into_iter()
        .map(|rec| rec.id);

    for node in device_ids {
        for (prop_name, db_col, dev_class, unit) in ALWAYS_SUPPORTED_PROPS {
            sensors.push(generate_entry(
                db_url, node, prop_name, db_col, dev_class, unit,
            ));
        }

        for (prop_name, db_col, dev_class, unit) in SENSOR_PROPS {
            if !check_prop_support(&mut db, node, db_col).await? {
                continue;
            }

            sensors.push(generate_entry(
                db_url, node, prop_name, db_col, dev_class, unit,
            ));
        }

        if include_stats {
            for (prop_name, db_col, dev_class, unit) in SENSOR_STATS.iter().copied() {
                sensors.push(generate_stat_entry(
                    db_url, node, prop_name, db_col, dev_class, unit,
                ));
            }
        }
    }

    if verify && !verify_queries(&mut db, &sensors).await? {
        exit(1);
    }

    Ok(HassConfig {
        sql: SqlIntegrationConfig(sensors),
    })
}

async fn verify_queries(
    db: &mut PgConnection,
    sensors: &[SensorEntry],
) -> Result<bool, Box<dyn Error>> {
    for entry in sensors {
        let sql = &entry.query;
        let Err(why) = sqlx::query(sql).fetch_one(&mut *db).await else {
            continue;
        };

        if matches!(why, sqlx::Error::RowNotFound) {
            continue;
        }

        eprintln!(
            "Failed to verify query for sensor `{}` ({why}):\n{}\n",
            entry.name,
            sql_fmt(sql.clone()),
        );
        return Ok(false);
    }

    Ok(true)
}

async fn check_prop_support(
    db: &mut PgConnection,
    node: i16,
    col: &str,
) -> Result<bool, Box<dyn Error>> {
    let count = sqlx::query(&format!(
        "SELECT CAST(COUNT({col}) AS INT8) FROM measurements WHERE {col} IS NOT NULL AND node = {node};"
    ))
    .fetch_one(db)
    .await?;
    let count = count.get::<i64, _>(0);

    Ok(count > 0)
}

fn generate_entry(
    db_url: &str,
    node_id: i16,
    prop_name: &str,
    col: &str,
    dev_class: &str,
    unit: &str,
) -> SensorEntry {
    SensorEntry {
        db_url: db_url.to_string(),
        name: format!("Node #{node_id} {prop_name}"),
        query: format!(
            "SELECT {col} FROM measurements WHERE node = {node_id} ORDER BY id DESC LIMIT 1;"
        ),
        column: col.to_string(),
        device_class: if dev_class.is_empty() {
            None
        } else {
            Some(dev_class.to_string())
        },
        unit_of_measurement: if unit.is_empty() {
            None
        } else {
            Some(unit.to_string())
        },
    }
}

fn generate_stat_entry(
    db_url: &str,
    node_id: i16,
    prop_name: &str,
    col: &str,
    dev_class: Option<&str>,
    unit: Option<&str>,
) -> SensorEntry {
    SensorEntry {
        db_url: db_url.to_string(),
        name: format!("Node #{node_id} {prop_name}"),
        query: sql_fmt(format!(
            "SELECT statistics.{col}
            FROM statistics
            INNER JOIN measurements ON measurements.id = statistics.measurement
            WHERE measurements.node = {node_id}
            ORDER BY measurements.id DESC LIMIT 1;"
        )),
        column: col.to_string(),
        device_class: dev_class.map(ToString::to_string),
        unit_of_measurement: unit.map(ToString::to_string),
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sql_fmt(sql: String) -> String {
    sql.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\n', "")
}
