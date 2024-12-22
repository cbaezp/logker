use logker::{cli, config, logging, monitoring};
use nix::unistd::daemon;

fn main() {
    logging::init_logging().expect("Failed to initialize logging");

    let args = cli::parse_args();
    let config = config::load_config(&args.config_path).expect("Failed to load configuration");

    if args.daemonize {
        daemon(true, false).expect("Failed to daemonize");
    }

    monitoring::start_monitoring(config).expect("Monitoring failed");
}
