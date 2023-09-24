use dicegame::expr::lex;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let lexed = lex::lex(input.as_str());
    println!("{lexed:?}");
}
