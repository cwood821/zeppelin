extern crate reqwest;
use reqwest::Client;

pub fn check(urls: Vec<String>) -> Vec<String> {
  let mut failures = Vec::new();

  for url in urls {
    let client = Client::new();
    let _ = client
        .head(&url)
        .send()
        .map(|r| {
          r.status().is_success()
        })
        .map_err(|_| {
          failures.push(url.clone());
          false
      });
    }
      
  failures
}
