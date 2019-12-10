//TODO config to specify file path

use std::fs::File;
use std::io::prelude::*;

pub fn generate_template() {
  println!("Got it. Generating template now ...");
  let result = create_file();
}

fn create_file() -> std::io::Result<()> {
  let mut file = File::create("foo.txt")?;
  file.write_all(
    b"---
  title: 'Title'
  date: '2019-02-07'
  ---",
  )?;
  Ok(())
}
