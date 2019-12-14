use std::env;
use std::fs;
use std::io::prelude::*;

pub fn generate(name: &str) {
  println!("Got it. Generating template now ...");
  create_file(&name).expect("Issue generating template.");
}

fn create_file(name: &str) -> std::io::Result<()> {
  let filename = format!("{}.md", name);
  let mut file = fs::File::create(filename)?;
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
    let project_dir: Result<std::path::PathBuf, std::path::PathBuf> =
      Ok(env::current_dir().expect("whoops"));
    let path = format!(
      "{}/test.md",
      project_dir.unwrap().to_str().unwrap().to_string()
    );
    assert_eq!((), generate("test"));
    assert_eq!(true, fs::metadata(path).is_ok());
  }

  #[test]
  fn create_file_works() {
    assert_eq!(true, create_file("ward").is_ok())
  }
}
