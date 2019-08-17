use std::env;

mod status_check;
use status_check::StatusCheck;

mod url_repository;
use url_repository::UrlRepository;

mod notifier;
use notifier::Notifier;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // TODO: Error handling for args
    let url_file = &args[1];
    let slack_url = &args[2];

    let notifier = Notifier {
        url: slack_url.to_string()
    };

    let urls = match UrlRepository::from_file(url_file) {
        Ok(urls) => urls,
        Err(_) => panic!("No urls in the urls file") 
    };

    for url in &urls {
        match StatusCheck::is_ok(&url) {
           true => {},
           false => {
                let message = format!("🚨 {} is down!", url);
                notifier.notify(&message);
           } 
        }
    }

    Ok(())
}