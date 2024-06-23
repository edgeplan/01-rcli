use clap::Parser;
use rcli::{process_csv, Commands, Opts};

fn main() -> anyhow::Result<()> {
    unsafe { backtrace_on_stack_overflow::enable() };
    let opts = Opts::parse();

    match opts.command {
        Commands::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
