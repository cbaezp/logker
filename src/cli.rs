use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "logker")]
pub struct Args {

    #[arg(long, short, default_value = "config/logker_config.toml")]
    pub config_path: String, 

    /// Run the application as a daemon
    #[arg(long, short)]
    pub daemonize: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
