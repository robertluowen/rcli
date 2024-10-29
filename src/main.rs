use anyhow::Result;
use clap::Parser;
use rcli::{process::process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts2) => {
            let output = if let Some(output) = opts2.output {
                output.clone()
            } else {
                // "output.csv".into()
                format!("output.{}", opts2.format)
            };
            process_csv(&opts2.input, output, opts2.format)?;
        }
        SubCommand::GenPass(opts) => {
            println!("Generate password : {:?}", opts);
        }
    }
    Ok(())
}
