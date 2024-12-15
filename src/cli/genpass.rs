use crate::{gen_pass, CmdExecutor};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = gen_pass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
        )?;
        println!("{}", password);
        let estimate = zxcvbn::zxcvbn(&password, &[]);
        eprintln!("password strong: {}", estimate.score());
        Ok(())
    }
}
