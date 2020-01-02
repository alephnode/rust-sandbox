extern crate chrono;
use chrono::{DateTime, Utc};
use std::fs;
use std::io::prelude::*;

pub fn generate(article_info: Vec<String>) {
  let name = &article_info[0];
  let title = &article_info[1];
  println!("Got it. Generating template now ...");
  create_file(&name, &title).expect("Issue generating template.");
}

fn create_file(name: &str, title: &str) -> std::io::Result<()> {
  let filename = format!("{}.md", name);
  let now: DateTime<Utc> = Utc::now();
  let mut file = fs::File::create(filename)?;
  file.write_all(
    format!(
      "
  ---
  title: {}
  date: {}
  ---
  ",
      title,
      now.format("%b %e %Y")
    )
    .as_bytes(),
  )?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn generate_works() {
    let project_dir: Result<std::path::PathBuf, std::path::PathBuf> =
      Ok(env::current_dir().expect("whoops"));
    let path = format!(
      "{}/test.md",
      project_dir.unwrap().to_str().unwrap().to_string()
    );
    let example_article = vec!["test".to_string(), "test me".to_string()];
    assert_eq!((), generate(example_article));
    assert_eq!(true, fs::metadata(path).is_ok());
  }
}
