mod io_helper;
mod template_helper;

fn main() {
    let article_name = String::from(io_helper::get_article_name());

    // TODO: write confirm handler
    io_helper::confirm(&article_name);

    template_helper::generate_template(&article_name);
}
