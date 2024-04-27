use phystech_radar::*;

fn main() {
    // let input = std::fs::read_to_string("input.txt").unwrap();
    // print_matrix_csv(&input);
    // print_weights_csv(&input);

    // println!("{}", 0.99997f32.powi(100000));

    // println!("{matrix}");
    // println!("{weights:?}");

    let input = std::fs::read_to_string("input28.csv").unwrap();
    let (matrix, weights) = parse_input_csv(&input);

    // println!("{}", weights.iter().sum::<f64>() / weights.len() as f64);

    for (hypot, weight) in search_sa2(&matrix, &weights) {
        println!("{hypot:?} | {weight:.3}");
    }
}
