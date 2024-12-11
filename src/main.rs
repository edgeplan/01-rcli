use clap::Parser;
use rcli::{
    gen_pass, process_csv, process_decode, process_encode, Base64SubCommand, Command, Opts,
};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.command {
        Command::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
        Command::GenPass(opts) => {
            gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
        }
        Command::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
