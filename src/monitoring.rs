use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event};
use std::sync::mpsc::channel;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn start_monitoring(config: crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    for log_file in &config.log_files {
        watcher.watch(Path::new(log_file), RecursiveMode::NonRecursive)?;
    }

    let failed_login_threshold = config.failed_login_threshold;

    for Event { paths, .. } in rx.into_iter().flatten() {
        for path in paths {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);

            let mut failed_logins = 0;

            for line in reader.lines() {
                let line = line?;
                if line.contains("Failed login") {
                    failed_logins += 1;
                    if failed_logins > failed_login_threshold {
                        println!("Too many failed logins detected.");
                    }
                }
            }
        }
    }

    Ok(())
}
