use phystech_radar::*;
use std::env;

fn main() {
    let path = env::args().nth(1).unwrap_or("input.csv".into());

    let input = match std::fs::read_to_string(&path) {
        Ok(string) => string,
        Err(err) => {
            eprintln!("error opening {path}: {err}");
            return;
        }
    };

    let (matrix, weights) = parse_input(&input);
    let answers = search(&matrix, &weights);
    print_answer(&answers);
}
