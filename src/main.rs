use std::path::Path;

use clap::{Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};
use anyhow;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "csv", about = "show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    //defaul_value  需要传入实现了from trait的类型
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)] //defaul_value_t  需要传入literal
    header: bool,
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("file not exists".to_string())
    }
}
fn main()->anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.command {
        Commands::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret=Vec::with_capacity(128);
            for result in reader.deserialize(){
                let reocrd:Player = result?;
                ret.push(reocrd);
            }
            let json=serde_json::to_string_pretty(&ret)?;
            std::fs::write(opts.output, json)?;
            // let records = reader
            //     .deserialize()
            //     .map(|record| record.unwrap())
            //     .collect::<Vec<Player>>();
            // println!("{:?}", records);
        }
    }
    Ok(())
}
