mod io_pal;

fn main() {
    let article_name = String::from(io_pal::ask_question());

    io_pal::confirm(article_name);

    // TODO: conditional based on response
    println!("\nCool, I'll create that now ...");
}
