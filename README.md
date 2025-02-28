# Logker - Test

Lightweight daemon for monitoring logs and blocking activity on a Linux server.

## Configuration

Before running the application, ensure you have a configuration file at `config/logker_config.toml`. You can edit this file using any text editor like `vim` or `nano`:

```bash
# Edit configuration file
nano config/logker_config.toml
# Or
vim config/logker_config.toml
```

### Example Configuration
```toml
log_files = ["/var/log/auth.log"]  # Path to the log files to monitor
failed_login_threshold = 5         # Number of failed logins before blocking an IP
port_scan_threshold = 10           # Port scan attempts before blocking an IP
ban_duration = 3600                # Ban duration in seconds
syn_ack_response = true            # Send SYN-ACK response
```

## Running Logker

After setting up the configuration file, you can run Logker:

```bash
cargo run -- --config-path config/logker_config.toml
```

To run it as a daemon:

```bash
cargo run -- --config-path config/logker_config.toml --daemonize
```

To stop:

```bash
sudo pkill logker
```
---

### **Usage**

1. Clone the repository to the Linux server:
   ```bash
   git clone https://github.com/cbaezp/logker.git
   cd logker
   ```

2. Follow the steps in the `README.md` to configure and run the application. 

