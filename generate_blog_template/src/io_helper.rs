use std::io;

pub fn get_article_name() -> String {
  println!("\nName of the blog article: ");

  return format_name(handle_input());
}

pub fn confirm(article_name: &str) -> String {
  println!("\nYou Typed: {}", article_name);
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
