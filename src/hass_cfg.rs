use serde::Serialize;

#[derive(Serialize)]
pub struct HassConfig {
    pub sql: SqlIntegrationConfig,
}

#[derive(Serialize)]
pub struct SqlIntegrationConfig(pub Vec<SensorEntry>);

#[derive(Serialize)]
pub struct SensorEntry {
    pub db_url: String,
    pub name: String,
    pub query: String,
    pub column: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<String>,
}
