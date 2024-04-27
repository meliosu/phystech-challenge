use rand::{Rng, SeedableRng};
use std::collections::HashSet;

const SIZE: usize = 28;

pub fn search_sa2(matrix: &CompatibilityMatrix, weights: &Vec<f64>) -> Vec<(Vec<usize>, f64)> {
    const INITIAL_TEMP: f64 = 100.0;
    const MAGIC: f64 = 4.0;

    let mut masks: [u64; SIZE] = [0; SIZE];

    for i in 0..SIZE {
        for j in 0..SIZE {
            if matrix.matrix[i][j] {
                masks[i] |= 1u64 << j;
            }
        }
    }

    let mut answers: Vec<(u64, f64)> = vec![Default::default(); 8];
    let mut answer = 0u64;
    let mut weight = 0f64;
    let mut score = 0f64;
    let mut collisions = 0;
    let mut temperature = INITIAL_TEMP;
    let mut rng = rand::rngs::StdRng::from_entropy();

    while temperature > 0.001 {
        let th = rng.gen_range(0..SIZE);
        let th_bit = 1u64 << th;

        let mut new_collisions = collisions;
        let mut new_weight = weight;

        if answer & th_bit != 0 {
            new_weight -= weights[th];
            new_collisions -= (answer & !masks[th]).count_ones();
        } else {
            new_weight += weights[th];
            new_collisions += (answer & !masks[th]).count_ones();
        }

        let new_score = new_weight - new_collisions as f64 * MAGIC;

        if new_score > score || rng.gen::<f64>() < ((new_score - score) / temperature).exp() {
            if answer & th_bit != 0 {
                answer &= !th_bit;
            } else {
                answer |= th_bit;
            }

            score = new_score;
            weight = new_weight;
            collisions = new_collisions;

            if collisions == 0 {
                if answers.iter().any(|&(_, w)| w < weight) {
                    // if answer is not a child
                    if !answers.iter().any(|(a, _)| answer & a == answer) {
                        // check if answer is a parent
                        answers = answers
                            .into_iter()
                            .filter(|&(a, _)| a & answer != a)
                            .collect();

                        answers.push((answer, weight));
                        answers.sort_by(|(_, w1), (_, w2)| w2.total_cmp(&w1));
                        answers.resize(8, Default::default());
                    }
                }
            }
        }

        temperature *= 0.9999975;
    }

    answers
        .iter()
        .take(5)
        .map(|(a, w)| ((0..SIZE).filter(|th| a & (1u64 << th) != 0).collect(), *w))
        .collect()
}

pub fn search_sa(matrix: &CompatibilityMatrix, weights: &Vec<f32>) -> Vec<(HashSet<usize>, f32)> {
    const INITIAL_TEMP: f32 = 100.0;
    const MAGIC: f32 = 3.5;

    let mut answers: Vec<(HashSet<usize>, f32)> = vec![Default::default(); 5];

    let mut answer: HashSet<usize> = HashSet::new();
    let mut weight = 0f32;
    let mut score = 0f32;
    let mut collisions = 0;
    let mut temperature = INITIAL_TEMP;

    let mut rng = rand::rngs::StdRng::from_entropy();

    while temperature > 0.001 {
        let th = rng.gen_range(0..SIZE);

        let mut new_collisions = collisions;
        let mut new_weight = weight;

        if answer.contains(&th) {
            new_weight -= weights[th];

            for &i in answer.iter() {
                if !matrix.matrix[th][i] {
                    new_collisions -= 1;
                }
            }
        } else {
            new_weight += weights[th];

            for &i in answer.iter() {
                if !matrix.matrix[th][i] {
                    new_collisions += 1;
                }
            }
        }

        let new_score = new_weight - new_collisions as f32 * MAGIC;

        if new_score > score || rng.gen::<f32>() < ((new_score - score) / temperature).exp() {
            if answer.contains(&th) {
                answer.remove(&th);
            } else {
                answer.insert(th);
            }

            score = new_score;
            weight = new_weight;
            collisions = new_collisions;

            if collisions == 0 {
                if let Some(pos) = answers.iter().position(|&(_, w)| w < weight) {
                    if !answers.iter().any(|(a, _)| *a == answer) {
                        answers.insert(pos, (answer.clone(), weight));
                        answers.resize(5, Default::default());
                    }
                }
            }
        }

        // 0.99997
        temperature *= 0.99975;
    }

    answers
}

pub fn search_recursive(
    matrix: &CompatibilityMatrix,
    weights: &Vec<f32>,
    hypot: Vec<usize>,
    weight: f32,
) -> (Vec<usize>, f32) {
    let mut final_answer = hypot.clone();
    let mut final_weight = weight;

    'outer: for v in (0..SIZE).filter(|x| !hypot.contains(x)) {
        for &i in hypot.iter() {
            if !matrix.matrix[v][i] {
                continue 'outer;
            }
        }

        let mut new_hypot = hypot.clone();
        new_hypot.push(v);

        let (new_answer, new_weight) =
            search_recursive(matrix, weights, new_hypot, weight + weights[v]);

        if new_weight > final_weight {
            final_weight = new_weight;
            final_answer = new_answer;
        }
    }

    (final_answer, final_weight)
}

pub fn parse_input_csv(input: &str) -> (CompatibilityMatrix, Vec<f64>) {
    let mut matrix = CompatibilityMatrix::default();
    let mut weights = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i == SIZE {
            weights = line.split(',').map(|n| n.parse::<f64>().unwrap()).collect();
            break;
        }

        for (j, b) in line.split(',').enumerate().skip(i + 1) {
            let b = match b {
                "1" => true,
                "0" => false,
                _ => unreachable!(),
            };

            matrix.matrix[i][j] = b;
            matrix.matrix[j][i] = b;
        }
    }

    for i in 0..SIZE {
        matrix.matrix[i][i] = true;
    }

    (matrix, weights)
}

// helper to convert format to csv
pub fn print_matrix_csv(input: &str) {
    let mut matrix = CompatibilityMatrix::default();

    for (i, line) in input.lines().enumerate().take(SIZE) {
        for (j, b) in line
            .chars()
            .filter(|&c| c == '0' || c == '1')
            .enumerate()
            .skip(i + 1)
        {
            let b = match b {
                '1' => true,
                '0' => false,
                _ => unreachable!(),
            };

            matrix.matrix[i][j] = b;
            matrix.matrix[j][i] = b;
        }
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{},", if matrix.matrix[i][j] { 1 } else { 0 });
        }

        println!();
    }
}

pub fn print_weights_csv(input: &str) {
    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<f32>().unwrap())
        .for_each(|n| print!("{n},"));
}

#[derive(Default)]
pub struct CompatibilityMatrix {
    matrix: [[bool; SIZE]; SIZE],
}

impl std::fmt::Display for CompatibilityMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..SIZE {
            for j in 0..SIZE {
                write!(f, "{} ", if self.matrix[i][j] { 1 } else { 0 })?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
