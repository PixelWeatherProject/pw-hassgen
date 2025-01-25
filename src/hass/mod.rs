use serde::Serialize;
use serde_with::SerializeDisplay;
use strum::Display;

#[derive(Serialize)]
pub struct Entity {
    name: String,
    query: String,
    column: String,
    db_url: String,
    unit_of_measurement: Option<String>,
    device_class: Option<DeviceClass>,
    state_class: Option<StateClass>,
    icon: String,
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
