use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CrackCommand {
    /// The first argument
    pub hash: String,
}