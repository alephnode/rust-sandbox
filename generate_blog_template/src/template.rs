use std::fs::File;
use std::io::prelude::*;

pub fn generate(name: &str) {
  println!("Got it. Generating template now ...");
  create_file(&name).expect("Issue generating template.");
}

fn create_file(name: &str) -> std::io::Result<()> {
  let filename = format!("{}.md", name);
  let mut file = File::create(filename)?;
  file.write_all(
    b"---
  title: 'Title'
  date: '2019-02-07'
  ---",
  )?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generate_works() {
    assert_eq!((), generate("ward"))
  }

  #[test]
  fn create_file_works() {
    assert_eq!(true, create_file("ward").is_ok())
  }
}
