use logker::monitoring;
use logker::config::Config;
use std::fs::File;
use std::io::Write;

#[test]
fn test_start_monitoring() {
    let test_log_path = "/tmp/test.log";
    let mut test_log = File::create(test_log_path).expect("Failed to create test log file");
    writeln!(test_log, "Failed login attempt").expect("Failed to write to test log file");

    let config = Config {
        log_files: vec![test_log_path.to_string()],
        failed_login_threshold: 1,
        port_scan_threshold: 0,
        ban_duration: 0,
        syn_ack_response: false,
        custom_payload: None,
    };

    let result = monitoring::start_monitoring(config);
    assert!(result.is_ok());

    std::fs::remove_file(test_log_path).expect("Failed to remove test log file");
}
