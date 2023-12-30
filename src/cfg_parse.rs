use serde::{Deserialize, Serialize};

#[allow(clippy::struct_field_names)]
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}
