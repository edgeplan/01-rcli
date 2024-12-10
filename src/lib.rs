mod opts;
mod process {
    pub mod csv_convert;
    pub mod gen_pass;
}
pub use opts::{Command, Opts};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::gen_pass;
