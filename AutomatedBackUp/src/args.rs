use clap::{Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
pub struct BackUpArgs {
    #[clap(subcommand)]
    pub option: BackUpOption,
}

#[derive(Debug, Subcommand, Clone)]
pub enum BackUpOption {
    /// Start the backup process
    Start,
    /// Stop the backup process
    Stop,
    /// Displays the logs
    Logs,
    /// Displays the process status
    Status,
    /// Configure settings (source directory, destination directory, backup interval)
    Config,
    /// Restore a backup
    Restore(RestoreSubcommand),
}

#[derive(Debug,Parser, Clone)]
pub struct RestoreSubcommand {
    #[clap(subcommand)]
    pub command: RestoreCommand,
    #[clap(short, long, global = true)]
    pub location: Option<String>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum RestoreCommand {
    /// Restore the latest backup
    Latest,
    /// Restore a specific backup
    Custom,
}

