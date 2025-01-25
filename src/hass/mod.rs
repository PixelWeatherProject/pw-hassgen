use serde::Serialize;
use serde_with::SerializeDisplay;
use strum::Display;

#[derive(Serialize)]
pub struct Entity {
    name: String,
    query: String,
    column: &'static str,
    db_url: String,
    unit_of_measurement: Option<&'static str>,
    device_class: Option<DeviceClass>,
    state_class: Option<StateClass>,
    icon: &'static str,
}

#[derive(Display, SerializeDisplay)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum DeviceClass {
    Duration,
    Humidity,
    Temperature,
    SignalStrength,
    Voltage,
}

#[derive(Display, SerializeDisplay)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum StateClass {
    Measurement,
}

impl Entity {
    pub fn new_temperature(node_id: u16, db_url: &str) -> Self {
        Self {
            name: format!("Node {node_id} Temperature"),
            query: format!("SELECT temperature FROM measurements WHERE id = {node_id} ORDER BY \"when\" DESC LIMIT 1"),
            column: "temperature",
            db_url: db_url.to_string(),
            unit_of_measurement: Some("°C"),
            device_class: Some(DeviceClass::Temperature),
            state_class: Some(StateClass::Measurement),
            icon: "mdi:temperature"
        }
    }

    pub fn new_humidity(node_id: u16, db_url: &str) -> Self {
        Self {
            name: format!("Node {node_id} Humidity"),
            query: format!("SELECT humidity FROM measurements WHERE id = {node_id} ORDER BY \"when\" DESC LIMIT 1"),
            column: "humidity",
            db_url: db_url.to_string(),
            unit_of_measurement: Some("%"),
            device_class: Some(DeviceClass::Humidity),
            state_class: Some(StateClass::Measurement),
            icon: "mdi:humidity"
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}
