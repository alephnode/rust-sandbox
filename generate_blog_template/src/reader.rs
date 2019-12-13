use std::io;

pub fn get_article_name() -> String {
  println!("\nName of the blog article: ");

  return format_name(handle_input());
}

pub fn get_article_title() -> String {
  println!("\nName of the blog title: ");

  return format_name(handle_input());
}

pub fn confirm(article_name: &str, article_title: &str) -> String {
  println!("\nArticle Name You Typed: {}", article_name);
  println!("\nArticle Title You Typed: {}", article_title);
  println!("\nConfirm: y/N:  ");

  return handle_input();
}

fn handle_input() -> String {
  let mut response = String::new();

  io::stdin()
    .read_line(&mut response)
    .expect("Failed to read line");

  return response;
}

//TODO better formatting
fn format_name(mut name: String) -> String {
  name.pop();
  return name;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn format_name_works() {
    assert_eq!("war", format_name(String::from("ward")))
  }
}
