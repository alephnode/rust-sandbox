use std::io;

pub fn handle_input() -> Vec<String> {
  let article_name = String::from(get_file_name());
  let article_title = String::from(get_article_title());
  let mut res = confirm(&article_name, &article_title);

  res.pop();
  while res != "y" {
    return handle_input();
  }
  vec![article_name, article_title]
}

fn get_file_name() -> String {
  println!("\nName of file: ");

  format_name(read_input(), true)
}

fn get_article_title() -> String {
  println!("\nName of the blog title: ");

  format_name(read_input(), false)
}

fn confirm(article_name: &str, article_title: &str) -> String {
  println!("\nArticle Name You Typed: {}", article_name);
  println!("\nArticle Title You Typed: {}", article_title);
  println!("\nConfirm: y/N:  ");

  read_input()
}

fn read_input() -> String {
  let mut response = String::new();

  io::stdin()
    .read_line(&mut response)
    .expect("Failed to read line");

  response
}

// Research if there's a better pattern in Rust other than manually passing flag.
// - no function overloading
// - no default parameters
fn format_name(mut name: String, strip: bool) -> String {
  name.pop();
  if strip {
    name = name.replace(" ", "-");
  }
  name
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn format_name_works() {
    assert_eq!("war", format_name(String::from("ward"), false));
    assert_eq!(
      "this-should-have-no-spaces",
      format_name(String::from("this should have no spaces\n"), true)
    )
  }
}
