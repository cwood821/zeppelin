extern crate reqwest;
use futures::{stream, Future, Stream};
use reqwest::r#async::Client;
use tokio;
use crate::notifier::Notifier;

pub struct StatusCheck {
  pub client: Client,
  pub notifier: Notifier
}

impl StatusCheck {

  pub fn check(self, urls: Vec<String>) {
    const PARALLEL_REQUESTS: usize = 2;

    let responses = stream::iter_ok(urls)
        .map(move |url| {
            let client = Client::new();
            client
                .get(&url)
                .send()
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    let work = responses
        .for_each(move |r| {
            // Note: panics if a connection is failed to establish
            // Need to handle case where a domain is no longer hosted
            // https://docs.rs/tokio/0.1.22/tokio/runtime/struct.Builder.html
            match r.status().is_success() {
              true => {},
              false => {
                    let message = format!("🚨 {} is down!", r.url());
                    self.notifier.notify(&message);
              } 
            }
            println!("Got {} from {}", r.status(), r.url());
            Ok(())
        })
        .map_err(|e| panic!("Error while processing: {}", e));

      tokio::run(work);
  }

}