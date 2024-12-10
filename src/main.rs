use clap::Parser;
use rcli::{process_csv, Command, Opts};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    println!("{:?}", opt);
    match opt.command {
        Command::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
    }

    Ok(())
}
