# Automated Backup System

A robust and efficient file backup solution written in Rust, designed to provide reliable automated file synchronization and backup capabilities.

## Features

- 🔄 **Automated Synchronization**: Automatically sync files between source and destination directories
- ⏰ **Scheduled Backups**: Configure backup schedules using cron-like syntax
- 📊 **Progress Tracking**: Real-time progress monitoring and detailed logging
- 🔍 **File Change Detection**: Efficient detection of file modifications
- 🔒 **Error Recovery**: Automatic retry mechanisms and error handling
- 📝 **Detailed Logging**: Comprehensive logging of all operations
- 🔄 **Incremental Backups**: Only backup changed files to save time and space

## Prerequisites

- Rust 1.56.0 or higher
- Sufficient disk space for backups
- Appropriate file system permissions

## Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd AutomatedBackUp
```

2. Build the project:
```bash
cargo build --release
```

3. Install the binary (optional):
```bash
cargo install --path .
```

## Configuration

Create a `config.toml` file in your project directory:

```toml
[backup]
source = "/path/to/source"
destination = "/path/to/destination"
schedule = "0 0 * * *"  # Daily at midnight
retry_attempts = 3
log_level = "info"
```

## Usage

### Basic Commands

Start the backup process:
```bash
cargo run -- start
```

Stop the backup process:
```bash
cargo run -- stop
```

Check backup status:
```bash
cargo run -- status
```

View backup logs:
```bash
cargo run -- logs
```

### Configuration

Configure backup settings:
```bash
cargo run -- config
```

### Restore Operations

Restore the latest backup:
```bash
cargo run -- restore latest
```

Restore a specific backup:
```bash
cargo run -- restore custom
```

Restore from a specific location:
```bash
cargo run -- restore latest --location /path/to/backup
```

## Project Structure

```
AutomatedBackUp/
├── src/
│   ├── main.rs
│   ├── args.rs
│   ├── file_io.rs
│   ├── demon.rs
│   ├── encryption.rs
│   ├── consts.rs
│   ├── handle.rs
│   └── error.rs
├── BackUpInfo/
├── tests/
├── Cargo.toml
└── README.md
```

## Development

To run tests:
```bash
cargo test
```

To run with debug logging:
```bash
RUST_LOG=debug cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
