use serde::Serialize;
use serde_with::SerializeDisplay;
use strum::Display;

#[derive(Serialize)]
pub struct Entity {
    name: String,
    query: String,
    column: &'static str,
    db_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measurement: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_class: Option<DeviceClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_class: Option<StateClass>,
    icon: &'static str,
}

#[derive(Display, SerializeDisplay)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum DeviceClass {
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
            query: format!("SELECT CAST(temperature AS DECIMAL(4, 2)) FROM measurements WHERE node = {node_id} ORDER BY \"when\" DESC LIMIT 1;"),
            column: "temperature",
            db_url: db_url.to_string(),
            unit_of_measurement: Some("°C"),
            device_class: Some(DeviceClass::Temperature),
            state_class: Some(StateClass::Measurement),
            icon: "mdi:thermometer"
        }
    }

    pub fn new_humidity(node_id: u16, db_url: &str) -> Self {
        Self {
            name: format!("Node {node_id} Humidity"),
            query: format!("SELECT humidity FROM measurements WHERE node = {node_id} ORDER BY \"when\" DESC LIMIT 1;"),
            column: "humidity",
            db_url: db_url.to_string(),
            unit_of_measurement: Some("%"),
            device_class: Some(DeviceClass::Humidity),
            state_class: Some(StateClass::Measurement),
            icon: "mdi:water-percent"
        }
    }

    pub fn new_battery(node_id: u16, db_url: &str) -> Self {
        Self {
            name: format!("Node {node_id} Battery"),
            query: format!("SELECT CAST(battery AS DECIMAL(3, 2)) FROM statistics JOIN measurements ON measurements.id = statistics.id WHERE measurements.node = {node_id} ORDER BY \"when\" DESC LIMIT 1;"),
            column: "battery",
            db_url: db_url.to_string(),
            unit_of_measurement: Some("V"),
            device_class: Some(DeviceClass::Voltage),
            state_class: Some(StateClass::Measurement),
            icon: "mdi:battery",
        }
    }

    pub fn new_wifi_essid(node_id: u16, db_url: &str) -> Self {
        Self {
            name: format!("Node {node_id} WiFi ESSID"),
            query: format!("SELECT wifi_ssid FROM statistics JOIN measurements ON measurements.id = statistics.id WHERE measurements.node = {node_id} ORDER BY \"when\" DESC LIMIT 1;"),
            column: "wifi_ssid",
            db_url: db_url.to_string(),
            unit_of_measurement: None,
            device_class: None,
            state_class: None,
            icon: "mdi:wifi",
        }
    }

    pub fn new_wifi_rssi(node_id: u16, db_url: &str) -> Self {
        Self {
            name: format!("Node {node_id} WiFi Signal Strength"),
            query: format!("SELECT wifi_rssi FROM statistics JOIN measurements ON measurements.id = statistics.id WHERE measurements.node = {node_id} ORDER BY \"when\" DESC LIMIT 1;"),
            column: "wifi_rssi",
            db_url: db_url.to_string(),
            unit_of_measurement: Some("dBm"),
            device_class: Some(DeviceClass::SignalStrength),
            state_class: Some(StateClass::Measurement),
            icon: "mdi:signal-cellular-2",
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}
