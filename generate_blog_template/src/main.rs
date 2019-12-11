mod reader;
mod template;

fn main() {
    let article_name = String::from(reader::get_article_name());

    // TODO: write confirm handler
    reader::confirm(&article_name);

    template::generate_template(&article_name);
}
