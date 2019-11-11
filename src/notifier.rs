use std::collections::HashMap;

// #[derive(Clone)]
// pub struct Notifier {
//   pub url: String
// }

// impl Notifier {
pub fn notify(message: &str, url: &str) {
  let mut map = HashMap::new();
  map.insert("text", message);

    let client = reqwest::Client::new();

    // TODO: Error handling
    let res = client.post(url)
        .json(&map)
        .send();

    if res.is_err() {
      // TODO: Do some logging
    }
}  

// }