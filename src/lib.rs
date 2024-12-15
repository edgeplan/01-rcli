mod cli {
    pub mod base64;
    pub mod csv;
    pub mod genpass;
    pub mod http;
    pub mod opts;
    pub mod text;
}
mod process {
    pub mod b64;
    pub mod csv_convert;
    pub mod gen_pass;
    pub mod http_serve;
    pub mod text;
}
mod utils;

pub use cli::base64::{Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand};
pub use cli::csv::CsvOptions;
pub use cli::genpass::GenPassOpts;
pub use cli::http::{HttpServeOpts, HttpSubCommand};
pub use cli::opts::{verify_path, Command, Opts};
pub use cli::text::{SignOpts, TextKeyGenerateOpts, TextSignFormat, TextSubCommand, VerifyOpts};
use enum_dispatch::enum_dispatch;
pub use process::b64::{process_decode, process_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::gen_pass;
pub use process::http_serve::process_http_serve;
pub use process::text::{process_generate, process_text_sign, process_text_verify};
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
