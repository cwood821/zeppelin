use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;

#[derive(Debug)]
pub enum URLError {
    UnableToReadFile,
    ParseError
}

pub struct UrlRepository {} 

impl UrlRepository {

  pub fn from_file(path: &str) -> Result<Vec<String>, URLError> {
    let f = File::open(path).map_err(|_| URLError::UnableToReadFile)?;

    let file = BufReader::new(&f);
    let mut urls = Vec::new(); 
    for line in file.lines() {
        let l = line.map_err(|_| URLError::ParseError)?;
        urls.push(l);
    }      

    Ok(urls)
  }
}