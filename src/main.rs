mod status_check;
use status_check::{check};
mod url_parser;
use url_parser::parse_urls;
mod notifier;
use notifier::{notify};
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

    let failures = check(urls);
    failures.iter().for_each(move |failure| {
        let msg = format!("Failed to connect to {}", failure);
        println!("{}", msg);
        notify(&msg, &slack_url);
    });
}