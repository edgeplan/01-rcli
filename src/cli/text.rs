use crate::cli::opts::{verify_file, verify_path};
use crate::{process_generate, process_text_sign, process_text_verify, CmdExecutor};
use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key")]
    Sign(SignOpts),
    #[command(about = "Verify a signed message")]
    Verify(VerifyOpts),
    #[command(about = "generate a new key pair")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Parser, Debug)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value="blake3")]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "-")]
    pub sign: String,
    #[arg(long, value_parser = parse_format, default_value="blake3")]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path, default_value = "-")]
    pub output: PathBuf,
}

#[derive(Clone, Debug)]
pub enum TextSignFormat {
    Blake3,
    ED25519,
}

pub fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse::<TextSignFormat>()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            _ => Err(anyhow::Error::msg("unknown format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::ED25519 => write!(f, "ed25519"),
        }
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_generate(&self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                std::fs::write(name, &key[0])?;
            }
            TextSignFormat::ED25519 => {
                let name = self.output;
                std::fs::write(name.join("ed25519.sk"), &key[0])?;
                std::fs::write(name.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
}
impl CmdExecutor for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = process_text_verify(&self.input, &self.key, &self.sign, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

impl CmdExecutor for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_text_sign(&self.input, &self.key, self.format)?;
        Ok(())
    }
}
impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await?,
            TextSubCommand::Verify(opts) => opts.execute().await?,
            TextSubCommand::Generate(opts) => opts.execute().await?,
        }
        Ok(())
    }
}
