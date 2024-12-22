use tracing_subscriber::FmtSubscriber;

pub fn init_logging() -> Result<(), String> {
    FmtSubscriber::builder()
        .try_init()
        .map_err(|e| format!("Failed to initialize logging: {}", e))
}
