use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(rename = "jwt-secret")]
    pub jwt_secret: String,
    #[serde(rename = "max-payload-size")]
    pub max_payload_size: u64, // stored in MB, but you can convert later
    #[serde(rename = "folder-path")]
    pub folder_path: String,
}

impl ServerConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.is_empty() {
            return Err("Server host cannot be empty".to_string());
        }
        if self.port == 0 {
            return Err("Server port must be greater than 0".to_string());
        }
        if self.jwt_secret.is_empty() {
            return Err("JWT secret must not be empty".to_string());
        }
        if self.folder_path.is_empty() {
            return Err("Folder path cannot be empty".to_string());
        }
        Ok(())
    }
    pub fn max_payload_bytes(&self) -> usize {
        (self.max_payload_size << 20) as usize
    }
}
