use std::env;

use reqwest::r#async::Client; // 0.9.14
mod status_check;
use status_check::StatusCheck;
mod url_repository;
use url_repository::UrlRepository;
mod notifier;
use notifier::Notifier;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Two arguments required: url file and slack endpoint");
    }

    let url_file = &args[1];
    let slack_url = &args[2];

    let urls = match UrlRepository::from_file(url_file) {
        Ok(urls) => urls,
        Err(_) => panic!("Failed to retrieve urls from the url file") 
    };

    let notifier = Notifier {
        url: slack_url.to_string()
    };

    let status_check = StatusCheck {
        client: Client::new(),
        notifier: notifier
    };

    status_check.check(urls);
    
    Ok(())
}