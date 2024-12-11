mod cli {
    pub mod base64;
    pub mod csv;
    pub mod genpass;
    pub mod opts;
}
mod process {
    pub mod b64;
    pub mod csv_convert;
    pub mod gen_pass;
}
pub use cli::base64::Base64SubCommand;
pub use cli::opts::{Command, Opts};
pub use process::b64::{process_decode, process_encode};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::gen_pass;
