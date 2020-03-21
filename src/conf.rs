use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "zeppelin")]
pub struct Opt {
    /// URL to send HTTP POST request to when a checked url responds non-200 
    #[structopt(short = "w", long = "webhook", required(false), default_value="")]
    pub webhook_url: String,

    /// Files with urls, separated by new lines
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}