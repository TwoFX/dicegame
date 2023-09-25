use dicegame::expr::Expr;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parsed = Expr::parse(input.as_str());
    println!("{parsed:?}");
}
