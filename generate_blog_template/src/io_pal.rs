use std::io;

//TODO abstract out the common parts
pub fn get_article_name() -> String {
  let mut article_name = String::new();

  println!("\nName of the blog article: ");

  io::stdin()
    .read_line(&mut article_name)
    .expect("Failed to read line");

  return article_name;
}

pub fn confirm(guess: String) -> String {
  let mut confirmation = String::new();

  println!("\nYou Typed: {}", guess);
  println!("\nConfirm: y/N:  ");

  io::stdin()
    .read_line(&mut confirmation)
    .expect("Failed to read line");

  return confirmation;
}
