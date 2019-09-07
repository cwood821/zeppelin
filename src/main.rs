use reqwest::r#async::Client; // 0.9.14
mod status_check;
use status_check::StatusCheck;
mod url_parser;
use url_parser::parse_urls;
mod notifier;
use notifier::Notifier;
use structopt::StructOpt;
mod conf;


type Result<T> = std::result::Result<T, i32>;

fn main() -> Result<()> {
    let opt = conf::Opt::from_args();

    let url_file = opt.file;
    let slack_url = opt.webhook_url;

    let urls = match parse_urls(url_file.to_str().unwrap()) {
        Ok(urls) => urls,
        Err(_) => {
            panic!("Unable to parse urls from file {}", url_file.to_str().unwrap_or(""));
        } 
    };

    let notifier = Notifier {
        url: slack_url.to_string()
    };

    let status_check = StatusCheck {
        client: Client::new(),
        notifier: notifier,
        concurrency: opt.concurrency,
        debug: opt.debug
    };

     match status_check.are_ok(urls) {
         true => Ok(()),
         false => std::process::exit(1)
     }
}