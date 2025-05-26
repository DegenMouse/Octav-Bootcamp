use std::{io, io::Write, io::Read};
extern crate fs_extra;
use crate::consts::{ExcludeTypes, ReportInfo};
use crate::encryption::decrypt_large_file;
use fs_extra::remove_items;
use fs_extra::dir::get_size;
use std::fs;
use chrono::Local;
use std::fs::File;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipArchive;
use walkdir::WalkDir;
use colored::Colorize;
use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;

use crate::error::BackupResult;
use crate::consts::ConfigSettings;

macro_rules! warn_m {
    ($name:expr) => {
        format!("WARN: {}!", $name).yellow()
    };
}   

fn get_path_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    let line = inp.trim().to_string();
    match fs::metadata(line.clone()) {
        Ok(metadata) => {
            let permissions = metadata.permissions();
            if permissions.readonly() {
                println!("{}", warn_m!("Permission denied!"));
                get_path_input(text)
            } else {
                line
            }
        }
        Err(_) => {
            println!("{}", warn_m!("Path not found!"));
            get_path_input(text)
        }
    }
}

fn get_input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp.trim().to_string()
}

pub fn get_config_input() -> BackupResult<ConfigSettings> {
    let source = get_path_input("Enter the source path: ");
    let destination = get_path_input("Enter the destination path: ");
    let interval = get_input("Enter the backup minute interval: ").parse::<u32>()
        .map_err(|e| anyhow::anyhow!("{}", warn_m!(format!("Invalid interval: {}", e))))?;
    let info_path = get_path_input("Enter the info file path: ");
    let password = get_input("Enter the password for encryption: ");
    let exclude = get_path_input("Enter file patterns or directories to exclude from the backup, separated by space");

    let collected: Vec<String> = exclude.split(" ").map(|s| s.to_string()).collect();
    let exclude_types = make_exclude_list(collected);
    Ok(ConfigSettings {
        source,
        destination,
        interval,
        pid_file: format!("{}/app.pid", info_path),
        log_file: format!("{}/app.log", info_path),
        err_file: format!("{}/app.err", info_path),
        password,
        exclude: exclude_types,
        count: 1
    })
}

pub fn print_logs(config: &ConfigSettings) -> BackupResult<()> {
    let mut file = fs::File::open(&config.log_file)
        .map_err(|e| anyhow::anyhow!(format!("Failed to open log file: {}", e)))?;
    let mut logs = String::new();
    file.read_to_string(&mut logs)
        .map_err(|e| anyhow::anyhow!(format!("Failed to read logs: {}", e)))?;
    println!("{}", logs);
    Ok(())
}

pub fn get_config_from_json() -> BackupResult<ConfigSettings> {
    let config_path = "/Users/octavoprita/Documents/Projects/DegenLab/Octav-Bootcamp/Rust-Projects/AutomatedBackUp/BackUpInfo/config_user.json";
    let config_file = fs::File::open(config_path)?;
    let config: ConfigSettings = serde_json::from_reader(config_file)?;
    Ok(config)
}

pub fn save_config_to_json(config: &ConfigSettings) -> BackupResult<()> {
    let config_path = "/Users/octavoprita/Documents/Projects/DegenLab/Octav-Bootcamp/Rust-Projects/AutomatedBackUp/BackUpInfo/config_user.json";
    let config_file = fs::File::create(config_path)?;
    serde_json::to_writer_pretty(config_file, config)?;
    Ok(())
}

pub fn compress(source: String, destination: String, exclude: ExcludeTypes, count: i32) -> BackupResult<String>{
    let start_time = std::time::Instant::now();
    
    let source_path = Path::new(&source);
    let dest_path = Path::new(&destination);
    let count_str = count.to_string();
    let trimmed_count = if count_str.len() > 4 {
        &count_str[..count_str.len() - 4]
    } else {
        &count_str
    };
    let filename = format!("{}_{}_{}.zip", 
        trimmed_count,
        source_path.file_name().and_then(|n| n.to_str()).unwrap_or("backup"),
        Local::now().format("%Y-%m-%d-%H-%M"));

    let mut zip = zip::ZipWriter::new(File::create(dest_path.join(filename.clone()))?);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in walkdir::WalkDir::new(source_path) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(source_path)?.to_string_lossy();

        let should_exclude = exclude.file.iter().any(|pattern| path.to_string_lossy().contains(pattern));
        let should_exclude_directory = exclude.directory.iter().any(|pattern| path.to_string_lossy().contains(pattern));

        if path.is_file() && !should_exclude && !should_exclude_directory {
            zip.start_file(relative_path, options)?;
            zip.write_all(&fs::read(path)?)?;
        }
    }
    zip.finish()?;

    let file_size = get_size(dest_path.join(filename.clone()))?;
    let report_info = ReportInfo {
        size: file_size,
        compression_time: start_time.elapsed().as_micros().to_string(),
        skipped_files: exclude
    };
    let report_filename = filename.trim_end_matches(".zip");
    create_report(report_info, format!("{}/{}_report.txt", destination, report_filename))?;
    Ok(filename)
}

pub fn decompress(source: String, destination: String, config: ConfigSettings) -> BackupResult<()> {
    
    let source_without_enc = source.trim_end_matches(".enc");
    decrypt_large_file(&source, source_without_enc, config.password.clone())?;
    
    let mut archive = ZipArchive::new(File::open(source_without_enc)?)?;
    fs::create_dir_all(&destination)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(&destination).join(file.mangled_name());
        
        if let Some(p) = outpath.parent() {
            fs::create_dir_all(p)?;
        }
        if !file.name().ends_with('/') {
            io::copy(&mut file, &mut File::create(outpath)?)?;
        }
    }
    delete_file(source_without_enc.to_string())?;
    Ok(())
}

pub fn latest_backup(directory: String) -> BackupResult<String> {
    Ok(fs::read_dir(directory)?
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().map(|e| e == "enc").unwrap_or(false))
        .max_by_key(|f| f.metadata().and_then(|m| m.created()).ok())
        .ok_or_else(|| anyhow::anyhow!("No zip files found"))?
        .path()
        .to_string_lossy()
        .into_owned())
}

pub fn restore_latest_backup(config: ConfigSettings) -> BackupResult<()> {
    let latest_backup = latest_backup(config.destination.clone())?;
    decompress(latest_backup, config.source.clone(), config)?;
    Ok(())
}

pub fn restore_custom_backup(config: ConfigSettings, option: Option<String>) -> BackupResult<()> {
    let backup_nr = get_input("Enter the backup number to restore: ");
    for entry in WalkDir::new(&config.destination).into_iter().filter_map(|e| e.ok()) {
        if entry.path().file_name()
            .and_then(|n| n.to_str())
            .filter(|n| n.starts_with(&backup_nr))
            .is_some() {
                if let Some(location) = option {
                    decompress(entry.path().to_string_lossy().to_string(), location, config)?;
                } else {
                    decompress(entry.path().to_string_lossy().to_string(), config.source.clone(), config)?;
                }
                return Ok(());
        }
    }
    Ok(())
}

pub fn make_exclude_list(config: Vec<String>) -> ExcludeTypes {
    let mut file_list = Vec::new();
    let mut directory_list = Vec::new();
    
    for exclude in config {
        if exclude.ends_with('*') {
            directory_list.push(exclude.trim().to_string());
        } else {
            file_list.push(exclude.trim().to_string());
        }
    }
    
    ExcludeTypes {
        file: file_list,
        directory: directory_list
    }
}

pub fn create_report(info: ReportInfo, destination: String) -> BackupResult<()> {
    let report_file = fs::File::create(destination)?;
    serde_json::to_writer_pretty(report_file, &info)?;
    Ok(())
}

pub fn delete_file(path: String) -> BackupResult<()> {
    let from_paths = vec![path];
    remove_items(&from_paths)?;
    Ok(())
}

pub fn watch_file_changes(path: String, _destination: String, _exclude: ExcludeTypes, _count: i32) -> BackupResult<()> {
    let watch_path = Path::new(&path);
    println!("Starting file monitoring for path: {}", watch_path.display());
    
    let (tx, rx) = channel();
    let watch_path_owned = watch_path.to_path_buf();

    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(event) => tx.send(event).unwrap_or_else(|e| eprintln!("Error sending event: {}", e)),
            Err(e) => eprintln!("Watch error: {}", e),
        }
    })?;

    // !!! Watch the path recursively to monitor all subdirectories
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                if let notify::EventKind::Modify(notify::event::ModifyKind::Data(_)) = event.kind {
                    if let Some(modified_path) = event.paths.first() {
                        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        if let Ok(relative_path) = modified_path.strip_prefix(&watch_path_owned) {
                            println!("{} - File modified: {} at {}", 
                                "MODIFIED".yellow(),
                                relative_path.display(),
                                timestamp
                            );
                            //compress(path.clone(), _destination.clone(), _exclude.clone(), _count + 1)?;
                        }
                    }
                }
            },
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => continue,
                    RecvTimeoutError::Disconnected => {
                        eprintln!("Watch error: Channel disconnected");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}