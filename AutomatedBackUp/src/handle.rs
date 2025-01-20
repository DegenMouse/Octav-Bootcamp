use clap::Parser;
use colored::Colorize;

use crate::args::{BackUpArgs, BackUpOption, RestoreCommand};
use crate::file_io::{decompress, get_config_from_json, get_config_input, latest_backup, print_logs, restore_latest_backup, restore_custom_backup, save_config_to_json};
use crate::demon::{start_daemon, stop_daemon, check_daemon_status};
use crate::error::BackupResult;


macro_rules! info_m {
    ($name:expr) => {
        format!("INFO: {}", $name).green()
    };
}


pub fn flow() -> BackupResult<()> {
    let args = BackUpArgs::parse();
    let mut config = get_config_from_json()?;
    
    match args.option {
        BackUpOption::Start => start_daemon(&config),
        BackUpOption::Stop => stop_daemon(config),
        BackUpOption::Logs => print_logs(&config),
        BackUpOption::Status => {
            if check_daemon_status(&config)? {
                println!("{}", info_m!("Backup is active"));
            } else {
                println!("{}", info_m!("Backup is inactive"));
            }
            Ok(())
        },
        BackUpOption::Config => {
            config = get_config_input()?;
            save_config_to_json(&config)
        }
        BackUpOption::Restore(restore_subcommand) => {
            match restore_subcommand.command {
                RestoreCommand::Latest => {
                    if let Some(location) = restore_subcommand.location {
                        decompress(latest_backup(config.destination.clone())?, location, config.clone())
                    } else {
                        restore_latest_backup(config)
                    }
                }
                RestoreCommand::Custom => restore_custom_backup(config)
            }
        }
    }
}