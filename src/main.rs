// rcli csv =i input.csv -o output.json -header -d ','
use clap::Parser;
// use csv::Reader;
// use serde::{Deserialize, Serialize};
use std::path::Path;
#[derive(Debug, Parser)]
#[command(name ="rcli",version, author,about,long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show23 CSV ,or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Error creating")
    }
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
