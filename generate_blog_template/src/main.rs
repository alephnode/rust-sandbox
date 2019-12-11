mod io_pal;
mod template_builder;

fn main() {
    let article_name = String::from(io_pal::get_article_name());

    // TODO: write confirm handler
    io_pal::confirm(&article_name);

    template_builder::generate_template(&article_name);
}
