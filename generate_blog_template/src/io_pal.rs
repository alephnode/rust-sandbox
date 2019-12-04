use std::io;

//TODO abstract out the common parts
pub fn ask_question() -> String {
  let mut guess = String::new();

  println!("\nName of the blog article: ");

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  return guess;
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