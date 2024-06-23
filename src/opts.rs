use std::{fmt, path::Path, str::FromStr};

use clap::{Parser, Subcommand};

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
    #[command(name = "genpass", about = "generate password")]
    GeneratePassword(GeneratePasswordOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    //default_value  需要传入实现了from trait的类型
    pub output: Option<String>,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)] //default_value_t  需要传入literal
    pub header: bool,
    #[arg(long, value_parser=parse_format)]
    pub format: OutputFormat,
}

#[derive(Debug, Parser)]
pub struct GeneratePasswordOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(short, long, default_value_t = true)]
    pub number: bool,
    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("file not exists".to_string())
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => Err(anyhow::anyhow!("format not support {}", v)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

// 这么写会导致栈溢出 这是为啥？
// impl fmt::Display for OutputFormat {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{}", self.to_string())
// }}
