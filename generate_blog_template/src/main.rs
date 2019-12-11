mod io;
mod template;

fn main() {
    let article_name = String::from(io::get_article_name());

    // TODO: write confirm handler
    io::confirm(&article_name);

    template::generate_template(&article_name);
}
