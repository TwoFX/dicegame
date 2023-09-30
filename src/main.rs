use dicegame::expr::Expr;
use num_rational::Rational64;
use std::{collections::HashMap, io};

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{v:?}");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let h = histogram(v.as_slice());

    let parsed = match Expr::parse(input.as_str()) {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to parse: {e}");
            return;
        }
    };

    let hh = parsed.histogram();
    if h != hh {
        println!("Incorrect histogram: {hh:?}");
        return;
    }

    let one = Rational64::from_integer(1);
    let pe = match parsed.eval() {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to evaluate: {e}");
            return;
        }
    };
    if one != pe {
        println!("Incorrect value: {pe:?}");
    } else {
        println!("Correct!!");
    }
}

fn histogram(v: &[u32]) -> HashMap<u32, u32> {
    let mut res = HashMap::new();

    for x in v {
        *res.entry(*x).or_insert(0) += 1;
    }

    res
}
