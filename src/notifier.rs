use std::collections::HashMap;

pub struct Notifier {
  pub url: String
}

impl Notifier {
    pub fn notify(&self, message: &str) {
      let mut map = HashMap::new();
      map.insert("text", message);

        let client = reqwest::Client::new();
        // TODO: Error handling
        let res = client.post(&self.url)
            .json(&map)
            .send();
    }  
}