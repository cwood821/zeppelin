
mod status_check;
use status_check::StatusCheck;

mod url_repository;
use url_repository::UrlRepository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = match UrlRepository::from_file("urls.txt") {
        Ok(urls) => urls,
        Err(_) => panic!("No urls in the urls file") 
    };

    for url in &urls {
        let result = StatusCheck::is_ok(&url);
        println!("{} is ok: {:#?}", url, result);
    }

    Ok(())
}