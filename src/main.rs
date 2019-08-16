extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    let result = is_ok("https://christianwood.net");
    println!("{:#?}", result);
    Ok(())
}

fn is_ok(url: &str) -> bool {
   get_status(&url)
        .map_err(|_| false)
        .and_then(|status| Ok(status == reqwest::StatusCode::OK))
        .unwrap()
}

// TODO: remove hard dependency on reqwest types
fn get_status(url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let resp = reqwest::get(url)?;
    Ok(resp.status())
}

