use clap::Parser;
use dicegame::expr;
use dicegame::expr::Expr;
use num_rational::Rational64;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use std::{collections::HashMap, io, time::Instant};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    rounds: u32,

    #[arg(short, long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let mut rng = match &args.seed {
        Some(seed) => StdRng::seed_from_u64(*seed),
        None => StdRng::from_entropy(),
    };

    let mut total_incorrect_attempts = 0;

    let start = Instant::now();
    for _ in 0..args.rounds {
        let v = create(&mut rng);
        total_incorrect_attempts += play_round(v.as_slice());
    }

    println!(
        "Played {} rounds in {}s, {} incorrect attempts",
        args.rounds,
        start.elapsed().as_secs_f64(),
        total_incorrect_attempts
    );
}

fn create<T: Rng>(rng: &mut T) -> Vec<u32> {
    rng.sample_iter(Uniform::new(1, 7)).take(4).collect()
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

fn play_round(v: &[u32]) -> u32 {
    println!("{v:?}");

    let expected_histogram = histogram(v);
    let expected_value = Rational64::from_integer(1);

    let mut incorrect_attempts = 0;

    loop {
        if let Err(e) = process_input(&expected_histogram, &expected_value) {
            println!("Incorrect input: {e}");
            incorrect_attempts += 1;
            continue;
        } else {
            println!("Correct!");
            return incorrect_attempts;
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

    let expr = Expr::parse(input.as_str())?;

    let actual_histogram = expr.histogram();
    if actual_histogram != *expected_histogram {
        return Err(InputError::Histogram {
            expected: expected_histogram.clone(),
            actual: actual_histogram,
        });
    }

    let actual_value = expr.eval()?;
    if actual_value != *expected_value {
        return Err(InputError::Value {
            expected: *expected_value,
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
