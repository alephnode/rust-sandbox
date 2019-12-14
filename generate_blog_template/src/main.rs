mod reader;
mod template;

fn main() {
    let article_name = String::from(reader::get_file_name());
    let article_title = String::from(reader::get_article_title());
    // TODO: write confirm handler
    reader::confirm(&article_name, &article_title);

    template::generate(&article_name, &article_title);
}
