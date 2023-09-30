use dicegame::expr;
use dicegame::expr::Expr;
use num_rational::Rational64;
use rand::{distributions::Uniform, Rng};
use std::{collections::HashMap, io};
use thiserror::Error;

fn main() {
    for _ in 0..5 {
        let v = create();
        play_round(v.as_slice());
    }
}

fn create() -> Vec<u32> {
    rand::thread_rng()
        .sample_iter(Uniform::new(1, 7))
        .take(4)
        .collect()
}

#[derive(Debug, Error)]
enum InputError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] expr::ParseError),
    #[error(transparent)]
    Eval(#[from] expr::EvalError),
    #[error("incorrect histogram: expected {expected:?}, got {actual:?}")]
    Histogram {
        expected: HashMap<u32, u32>,
        actual: HashMap<u32, u32>,
    },
    #[error("incorrect value: expected {expected}, got {actual}")]
    Value {
        expected: Rational64,
        actual: Rational64,
    },
}

fn play_round(v: &[u32]) {
    println!("{v:?}");

    let expected_histogram = histogram(v);
    let expected_value = Rational64::from_integer(1);

    loop {
        if let Err(e) = process_input(&expected_histogram, &expected_value) {
            println!("Incorrect input: {e}");
            continue;
        } else {
            println!("Correct!");
            break;
        }
    }
}

fn process_input(
    expected_histogram: &HashMap<u32, u32>,
    expected_value: &Rational64,
) -> Result<(), InputError> {
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(InputError::Io(e));
    }

    let expr = Expr::parse(input.as_str()).map_err(InputError::Parse)?;

    let actual_histogram = expr.histogram();
    if actual_histogram != *expected_histogram {
        return Err(InputError::Histogram {
            expected: expected_histogram.clone(),
            actual: actual_histogram,
        });
    }

    let actual_value = expr.eval().map_err(InputError::Eval)?;
    if actual_value != *expected_value {
        return Err(InputError::Value {
            expected: expected_value.clone(),
            actual: actual_value,
        });
    }

    Ok(())
}

fn histogram(v: &[u32]) -> HashMap<u32, u32> {
    let mut res = HashMap::new();

    for x in v {
        *res.entry(*x).or_insert(0) += 1;
    }

    res
}
