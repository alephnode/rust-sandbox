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

  return format_name(read_input());
}

fn get_article_title() -> String {
  println!("\nName of the blog title: ");

  return format_name(read_input());
}

fn confirm(article_name: &str, article_title: &str) -> String {
  println!("\nArticle Name You Typed: {}", article_name);
  println!("\nArticle Title You Typed: {}", article_title);
  println!("\nConfirm: y/N:  ");

  return read_input();
}

fn read_input() -> String {
  let mut response = String::new();

  io::stdin()
    .read_line(&mut response)
    .expect("Failed to read line");

  return response;
}

fn format_name(mut name: String) -> String {
  name.pop();
  return name.replace(" ", "-");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn format_name_works() {
    assert_eq!("war", format_name(String::from("ward")));
    assert_eq!(
      "this-should-have-no-spaces",
      format_name(String::from("this should have no spaces\n"))
    )
  }
}
