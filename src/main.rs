use clap::Parser;
use rcli::{CmdExecutor, Opts};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opt = Opts::parse();
    opt.command.execute().await?;
    Ok(())
}
