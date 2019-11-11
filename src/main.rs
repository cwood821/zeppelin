use reqwest::r#async::Client; // 0.9.14
mod status_check;
use status_check::StatusCheck;
mod url_parser;
use url_parser::parse_urls;
mod notifier;
use structopt::StructOpt;
mod conf;

fn main() -> () {
    let opt = conf::Opt::from_args();

    let url_file = opt.file;
    let slack_url = opt.webhook_url.to_owned();

    let urls = match parse_urls(url_file.to_str().unwrap()) {
        Ok(urls) => urls,
        Err(_) => {
            panic!("Unable to parse urls from file {}", url_file.to_str().unwrap_or(""));
        } 
    };

    let status_check = StatusCheck {
        concurrency: opt.concurrency,
        debug: opt.debug
    };

    let failures = status_check.check(urls);
    println!("There were {} failures", failures.len());
    ()
    // .iter().for_each(move |failure| {
    //     format!("Failed to connect to {}", failure);
    //     Notifier::notify("Failed to connect to ", &slack_url);
    // })
}