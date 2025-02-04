use clap::Parser;
use colored::Colorize;
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct QuoteArgs {
    /// The number of quotes to print
    pub number: u32,
    /// Optional flag ([a] -author, [t]  -timestamp, [f] -print from file)
    pub first_flag: Option<String>,
    /// Optional flag ([a] -author, [t]  -timestamp, [f] -print from file)
    pub second_flag: Option<String>,
    /// Optional flag ([a] -author, [t]  -timestamp, [f] -print from file)
    pub third_flag: Option<String>,
}

/// Process the flags (AI generated solution)
impl QuoteArgs {
    pub fn process_flags(&self) -> (bool, bool, bool) {
        let mut flags = [false; 3]; // [author_flag, time_flag, file_flag]

        let mut flag_map = |flag: Option<&str>| match flag {
            Some("a") => flags[0] = true,
            Some("t") => flags[1] = true,
            Some("f") => flags[2] = true,
            Some(_) => println!("{}", "Invalid flag, refer to -- -help".red()),
            None => (),
        };

        [
            self.first_flag.as_deref(),
            self.second_flag.as_deref(),
            self.third_flag.as_deref(),
        ]
        .iter()
        .for_each(|&flag| flag_map(flag));

        (flags[0], flags[1], flags[2])
    }
}
