use clap::Parser;
use rcli::{process_csv, Commands, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.command {
        Commands::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
