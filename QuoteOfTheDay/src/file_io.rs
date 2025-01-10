use crate::quotes::Quote;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

use anyhow::Result;

pub fn read_quotes(file_path: &str) -> Result<Vec<Quote>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let quotes = serde_json::from_reader(reader)?;
    Ok(quotes)
}

pub fn write_quotes(file_path: &str, quotes: &[Quote]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, quotes)?;
    Ok(())
}
