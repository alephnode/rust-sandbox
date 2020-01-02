mod reader;
mod template;

fn main() {
    let template_info = reader::handle_input();
    template::generate(template_info);
}
