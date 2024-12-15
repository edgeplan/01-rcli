use crate::cli::base64::Base64SubCommand;
use crate::cli::csv::CsvOptions;
use crate::cli::genpass::GenPassOpts;
use crate::cli::text::TextSubCommand;
use crate::HttpSubCommand;
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "rcli")]
pub struct Opts {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum Command {
    #[command(name = "csv", about = "convert CSV file")]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "generate passphrase")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "print generated passphrase")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "sign a passphrase and verify it")]
    Text(TextSubCommand),
    #[command(subcommand, about = "http server")]
    Http(HttpSubCommand),
}
// impl CmdExecutor for Command {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             Command::Csv(opts) => opts.execute().await,
//             Command::GenPass(opts) => opts.execute().await,
//             Command::Base64(subcmd) => subcmd.execute().await,
//             Command::Text(subcmd) => subcmd.execute().await,
//             Command::Http(subcmd) => subcmd.execute().await,
//         }
//     }
// }

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file not found")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("Path is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("not-exists"), Err("Input file not found"));
    }
}
