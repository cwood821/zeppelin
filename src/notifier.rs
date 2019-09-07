use std::collections::HashMap;

pub struct Notifier {
  pub url: String
}

impl Notifier {
    pub fn can_notify(&self) -> bool {
      self.has_url()
    }

    pub fn has_url(&self) -> bool {
      !self.url.is_empty()
    }

    pub fn notify(&self, message: &str) {
      let mut map = HashMap::new();
      map.insert("text", message);

        let client = reqwest::Client::new();
        // TODO: Error handling
        let res = client.post(&self.url)
            .json(&map)
            .send();

        if res.is_err() {
          // Do some logging
          // https://crates.io/crates/flexi_logger
        }
    }  

}