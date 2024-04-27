use phystech_radar::*;
use std::env;

fn main() {
    let path = env::args().nth(1).unwrap_or("input.csv".into());

    let input = match std::fs::read_to_string(path) {
        Ok(string) => string,
        Err(err) => {
            eprintln!("error opening file: {err}");
            return;
        }
    };

    let (matrix, weights) = parse_input(&input);

    for (hypot, weight) in search(&matrix, &weights) {
        println!("{hypot:?} | {weight:.3}");
    }
}
