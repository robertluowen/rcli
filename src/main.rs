use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // "output.csv".into()
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
            Ok(())
        }
    }
}
