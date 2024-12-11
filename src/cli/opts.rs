use crate::cli::base64::Base64SubCommand;
use crate::cli::csv::CsvOptions;
use crate::cli::genpass::GenPassOpts;
use clap::{Parser, Subcommand};
use std::path::Path;

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
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("not-exists"), Err("Input file not found"));
    }
}
