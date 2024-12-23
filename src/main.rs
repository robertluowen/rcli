use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, SubCommand};

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
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                println!("encode: {:?}", opts);
            }
            Base64SubCommand::Decode(opts) => {
                println!("decode: {:?}", opts);
            }
        },
    }
    Ok(())
}
