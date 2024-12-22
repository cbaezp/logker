use logker::config::{load_config, Config};

#[test]
fn test_load_valid_config() {
    let toml_content = r#"
        log_files = ["/var/log/auth.log"]
        failed_login_threshold = 5
        port_scan_threshold = 10
        ban_duration = 3600
        syn_ack_response = true
        custom_payload = "Access denied!"
    "#;
    let config: Config = toml::from_str(toml_content).expect("Failed to parse config");

    assert_eq!(config.log_files, vec!["/var/log/auth.log"]);
    assert_eq!(config.failed_login_threshold, 5);
    assert!(config.syn_ack_response);
    assert_eq!(config.custom_payload.unwrap(), "Access denied!");
}

#[test]
fn test_load_missing_config_file() {
    let result = load_config("nonexistent.toml");
    assert!(result.is_err());
}
