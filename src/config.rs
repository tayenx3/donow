use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub data_path: String
}