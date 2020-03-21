use std::collections::HashMap;

pub fn notify(message: &str, url: &str) {
  let mut map = HashMap::new();
  map.insert("text", message);

  let client = reqwest::Client::new();

  let res = client.post(url)
      .json(&map)
      .send();

  if res.is_err() {
    // TODO: Do some logging
  }
}  
