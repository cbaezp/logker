use logker::cli;
use clap::Parser;

#[test]
fn test_parse_args() {
    let args = cli::Args::parse_from(["logker", "--config-path", "custom_config.toml", "--daemonize"]);
    assert_eq!(args.config_path, "custom_config.toml");
    assert!(args.daemonize);
}

#[test]
fn test_default_config_path() {
    let args = cli::Args::parse_from(["logker"]);
    assert_eq!(args.config_path, "config/logker_config.toml");
    assert!(!args.daemonize);
}
