use crate::cli::base64::Base64SubCommand;
use crate::cli::csv::CsvOptions;
use crate::cli::genpass::GenPassOpts;
use crate::cli::text::TextSubCommand;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "rcli")]
pub struct Opts {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "csv", about = "convert CSV file")]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "generate passphrase")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "print generated passphrase")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "sign a passphrase and verify it")]
    Text(TextSubCommand),
}

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
