use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSettings {
    pub source: String,
    pub destination: String,
    pub interval: u32,
    pub pid_file: String,
    pub log_file: String,
    pub err_file: String,
    pub password: String,
    pub exclude: ExcludeTypes,
    pub count: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExcludeTypes{
    pub file: Vec<String>,
    pub directory: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportInfo {
    pub size: u64,
    pub compression_time: String,
    pub skipped_files: ExcludeTypes
}
