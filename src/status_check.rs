extern crate reqwest;
use futures::{stream, future, Future, Stream};
use reqwest::Client;
use tokio;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use crate::notifier::notify;

pub struct StatusCheck {
  pub concurrency: usize,
  pub debug: bool
}

pub struct Result {
  url: String,
  status: reqwest::StatusCode,
}

impl StatusCheck {

  pub fn check(self, urls: Vec<String>) -> Arc<Vec<String>> {
    // loop over each url, 
    // spawn a group of threads
    // join/wait till they all finish 
    // filter if they were a success or not
    // let thread_handles = urls.iter().for_each(|url| thread::spawn()))

   let failures = Arc::new(Vec::new());
   let mut threads = vec![];

   for url in urls {
      let failures = Arc::clone(&failures);
      threads.push(thread::spawn(move || {
            let client = Client::new();
            let resp = client
                .head(&url)
                .send()
                .map(|r| {
                  r.status().is_success()
                })
                .map_err(|_| {
                  // Failed to connect case
                  false
                });
              
              match resp.is_ok() {
                true => {}
                false => {
                  eprintln!("{} is down or unreachable!", &url);
                  let msg = format!("{} is down!", &url);
                  notify(&msg, "localhost")
                } 
              }
      }));
   }

    for thread in threads {
        let _ = thread.join();
    }

    failures
  
    
    // let responses = stream::iter_ok(urls)
    //     .map(move |url| {
    //         let client = Client::new();
    //         // We use a head request because we only care
    //         // about the status, not the body of the response
    //         client
    //             .head(&url)
    //             .send()
    //             .and_then(|res| future::ok(Result{url: res.url().to_string(), status: res.status()}))
    //     })
    //     .buffer_unordered(self.concurrency);

    //   let work = responses.for_each(move |r| {
    //         if self.debug {
    //           println!("Got {} from {}", r.status, r.url);
    //         }

    //         match r.status.is_success() {
    //           true => {
    //             successes.push(r.url.to_string());
    //           },
    //           false => {} 
    //         }
    //         Ok(())
    //     })
    //     .map_err(|e| {
    //       // Avoid panics on connection failures
    //     });

    //   tokio::run(work);

      // vec!["yo".to_string()]
  }

}