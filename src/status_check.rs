extern crate reqwest;

pub struct StatusCheck {}

impl StatusCheck {

  pub fn is_ok(url: &str) -> bool {
    StatusCheck::get_status(&url)
          .map_err(|_| false)
          .and_then(|status| Ok(status == reqwest::StatusCode::OK))
          .unwrap()
  }

  // TODO: remove hard dependency on reqwest types
  pub fn get_status(url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
      let resp = reqwest::get(url)?;
      Ok(resp.status())
  }

}