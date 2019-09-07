use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "zeppelin")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// URL to send HTTP POST request to when a checked url responds non-200 
    #[structopt(short = "w", long = "webhook", required(false), default_value="")]
    pub webhook_url: String,

    /// Maximum number of concurrent HTTP requests
    #[structopt(short = "c", long = "--concurrency", default_value = "4")]
    pub concurrency: usize,

    /// Files with urls, separated by new lines
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}