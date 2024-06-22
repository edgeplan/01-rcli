use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "csv", about = "show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    //default_value  需要传入实现了from trait的类型
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)] //default_value_t  需要传入literal
    pub header: bool,
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("file not exists".to_string())
    }
}
