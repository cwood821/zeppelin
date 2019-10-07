extern crate reqwest;
use futures::{stream, Future, Stream};
use reqwest::r#async::Client;
use tokio;
use crate::notifier::Notifier;

pub struct StatusCheck {
  pub client: Client,
  pub notifier: Notifier,
  pub concurrency: usize,
  pub debug: bool
}

impl StatusCheck {

  pub fn are_ok(&'static self, urls: Vec<String>) -> bool {

    let responses = stream::iter_ok(urls)
        .map(move |url| {
            let client = Client::new();
            // We use a head request because we only care
            // about the status, not the body of the response
            client
                .head(&url)
                .send()
        })
        .buffer_unordered(self.concurrency);

    let mut all_ok = true;
 
    let work = responses
        .for_each(move |r| {
            match r.status().is_success() {
              true => {},
              false => {
                    let message = format!("🚨 {} is returning status {}!", r.url(), r.status());
                    self.report(&message);
                    all_ok = false;
              } 
            }

            if self.debug {
              println!("Got {} from {}", r.status(), r.url());
            }

            Ok(())
        })
        .map_err(move |_| {
          // TODO: Error handling for notification
          let message = format!("🚨 Failed to connect to a domain");
          self.report(&message);
        });

      tokio::run(work);

      all_ok
  }

  fn report(&self, message: &str) {
   if self.notifier.can_notify() {
      self.notifier.notify(&message);
    } 
  }


}