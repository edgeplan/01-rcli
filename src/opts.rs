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
}

#[derive(Parser, Debug)]
pub struct CsvOptions {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // "out_put.json.into()"
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file not found")
    }
}
