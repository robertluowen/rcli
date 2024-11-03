pub mod base64;
pub mod csv;
pub mod genpass;
pub use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name ="rcli",version, author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV ,or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}
