mod cli {
    pub mod base64;
    pub mod csv;
    pub mod genpass;
    pub mod opts;
    pub mod text;
}
mod process {
    pub mod b64;
    pub mod csv_convert;
    pub mod gen_pass;
    pub mod text;
}
mod utils;
pub use cli::base64::Base64SubCommand;
pub use cli::opts::{verify_path, Command, Opts};
pub use cli::text::{TextSignFormat, TextSubCommand};
pub use process::b64::{process_decode, process_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::gen_pass;
pub use process::text::{process_generate, process_text_sign, process_text_verify};
pub use utils::get_reader;
