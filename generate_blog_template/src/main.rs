mod io_pal;

fn main() {
    let guess = String::from(io_pal::ask_question());

    io_pal::confirm(guess);

    // TODO: conditional based on response
    println!("\nCool, I'll create that now ...");
}
