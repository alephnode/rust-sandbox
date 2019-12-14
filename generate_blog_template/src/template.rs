use std::env;
use std::fs;
use std::io::prelude::*;

pub fn generate(name: &str, title: &str) {
  println!("Got it. Generating template now ...");
  create_file(&name, &title).expect("Issue generating template.");
}

fn create_file(name: &str, title: &str) -> std::io::Result<()> {
  let filename = format!("{}.md", name);
  let mut file = fs::File::create(filename)?;
  file.write_all(
    format!(
      "
  ---
  title: {}
  date: '2019-02-08'
  ---
  ",
      title
    )
    .as_bytes(),
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
    assert_eq!((), generate("test", "test me"));
    assert_eq!(true, fs::metadata(path).is_ok());
  }
}
