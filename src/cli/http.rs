use crate::cli::opts::verify_path;
use clap::Parser;

use crate::{process_http_serve, CmdExecutor};
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}

// impl CmdExecutor for HttpSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             HttpSubCommand::Serve(opts) => opts.execute().await?,
//         }
//         Ok(())
//     }
// }
