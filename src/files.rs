use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn create_writer(filepath: &str) -> File {
  let path = Path::new(filepath);
  let display = path.display();
  match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why.description()),
      Ok(file) => file,
  }
}