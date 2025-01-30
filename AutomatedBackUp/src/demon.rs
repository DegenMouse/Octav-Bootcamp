use daemonize::Daemonize;
use std::fs;
use std::process::Command;
use chrono::Local;
use colored::Colorize;
use crate::error::BackupResult;
use crate::consts::ConfigSettings;
use crate::encryption::encrypt_large_file;
use crate::file_io::{compress, delete_file, save_config_to_json, get_config_from_json};


macro_rules! info_m {
    ($name:expr) => {
        format!("INFO: {}", $name).green()
    };
}


pub fn start_daemon(config: &ConfigSettings) -> BackupResult<()> {
    let stdout = fs::File::create(&config.log_file)
        .map_err(|e| anyhow::anyhow!("Failed to create log file: {}", e))?;
    let stderr = fs::File::create(&config.err_file)
        .map_err(|e| anyhow::anyhow!("Failed to create error file: {}", e))?;

    let daemonize = Daemonize::new()
        .pid_file(&config.pid_file)
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => {
            println!("Backup active");
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(run_background_task(config))?;
            Ok(())
        },
        Err(e) => {
            if e.to_string().contains("unable to lock pid file") {
                Err(anyhow::anyhow!("Backup is already running"))
            } else {
                Err(anyhow::anyhow!("Failed to start daemon: {}", e))
            }
        }
    }
}

pub fn stop_daemon(config: ConfigSettings) -> BackupResult<()> {
    if let Ok(pid_str) = fs::read_to_string(&config.pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            match Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .status()
            {
                Ok(status) if status.success() => {
                    println!("{}", info_m!("Daemon stopped."));
                    Ok(())
                },
                Ok(_) => Err(anyhow::anyhow!("Failed to stop daemon")),
                Err(err) => Err(anyhow::anyhow!(format!("Failed to stop daemon: {}", err))),
            }
        } else {
            Err(anyhow::anyhow!(format!("Invalid PID in {}", config.pid_file)))
        }
    } else {
        Err(anyhow::anyhow!(format!("PID file not found. Is the daemon running?")))
    }
}

pub async fn run_background_task(config: &ConfigSettings) -> BackupResult<()>{
    loop {

        let new_config = get_config_from_json()?;
        let mut dcount = new_config.count;
        dcount += 1;
        save_config_to_json(&ConfigSettings {
            source: config.source.clone(),
            destination: config.destination.clone(),
            exclude: config.exclude.clone(),
            password: config.password.clone(),
            interval: config.interval,
            count: dcount,
            log_file: config.log_file.clone(),
            err_file: config.err_file.clone(),
            pid_file: config.pid_file.clone(),
        })?;
        let filename = compress(config.source.clone(), config.destination.clone(), config.exclude.clone(), dcount)?;
        encrypt_large_file(&format!("{}/{}", config.destination, filename), &format!("{}/{}.enc", config.destination, filename), config.password.clone())?;
        let filename_without_enc = filename.trim_end_matches(".enc");
        delete_file(format!("{}/{}", config.destination, filename_without_enc))?;
        println!("Backup made at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        tokio::time::sleep(tokio::time::Duration::from_secs(60 * config.interval as u64)).await;
    }
}

pub fn check_daemon_status(config: &ConfigSettings) -> BackupResult<bool> {
    if let Ok(pid_str) = fs::read_to_string(&config.pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            // On Unix-like systems, sending signal 0 checks if process exists
            // Redirect stderr to /dev/null to suppress error messages
            match Command::new("kill")
                .arg("-0")
                .arg(pid.to_string())
                .stderr(std::process::Stdio::null())
                .status()
            {
                Ok(status) => Ok(status.success()),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}