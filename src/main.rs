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

    for (hypot, weight) in search_sa(&matrix, &weights) {
        println!("{hypot:?} | {weight}");
    }
}
