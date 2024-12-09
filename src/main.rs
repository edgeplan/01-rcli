use clap::Parser;
use rcli::{process_csv, Command, Opts};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    println!("{:?}", opt);
    match opt.command {
        Command::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }

    Ok(())
}
