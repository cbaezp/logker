use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_files: Vec<String>,
    #[allow(dead_code)] 
    pub failed_login_threshold: u32,
    #[allow(dead_code)] 
    pub port_scan_threshold: u32,
    #[allow(dead_code)] 
    pub ban_duration: u64,
    #[allow(dead_code)] 
    pub syn_ack_response: bool,
    #[allow(dead_code)] 
    pub custom_payload: Option<String>,
}

pub fn load_config<P: AsRef<std::path::Path>>(path: P) -> Result<Config, String> {
    let config_content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;
    toml::from_str(&config_content).map_err(|e| format!("Failed to parse config: {}", e))
}
