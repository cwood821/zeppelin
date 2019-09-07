use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;

#[derive(Debug)]
pub enum ParseError {
    UnableToReadFile,
    ParseError
}

pub fn parse_urls(path: &str) -> Result<Vec<String>, ParseError> {
  let f = File::open(path).map_err(|_| ParseError::UnableToReadFile)?;

  let file = BufReader::new(&f);
  let mut urls = Vec::new(); 
  for line in file.lines() {
      let l = line.map_err(|_| ParseError::ParseError)?;
      urls.push(l);
  }      

  Ok(urls)
}