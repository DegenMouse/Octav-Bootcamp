#[macro_use]
mod consts;
mod args;
mod error;
mod file_io;
mod demon;
mod encryption;
mod handle;

use colored::Colorize;

macro_rules! error_m {
    ($name:expr) => {
        format!("ERR: {}", $name).red()
    };
}

fn main() {
    match handle::flow() {
        Ok(_) => (),
        Err(e) => println!("{}", error_m!(format!("Error: {}", e))),
    }
}
