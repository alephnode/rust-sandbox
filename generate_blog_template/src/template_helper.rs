use std::fs::File;
use std::io::prelude::*;

pub fn generate_template(name: &str) -> u8 {
  println!("Got it. Generating template now ...");
  let result = create_file(&name);

  return 1;
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
