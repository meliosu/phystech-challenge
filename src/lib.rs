use rand::{rngs::StdRng, Rng, SeedableRng};

const SIZE: usize = 28;

pub fn search(matrix: &CompatibilityMatrix, weights: &Vec<f64>) -> Vec<(u64, f64)> {
    const INITIAL_TEMP: f64 = 300.0;
    const MULTIPLIER: f64 = 0.999975;

    let coeff = weights.iter().sum::<f64>() / weights.len() as f64;

    let masks: [u64; SIZE] = {
        let mut masks = [0; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                if matrix.matrix[i][j] {
                    masks[i] |= 1u64 << j;
                }
            }
        }

        masks
    };

    let mut answers: Vec<(u64, f64)> = vec![Default::default(); 8];
    let mut answer = 0u64;
    let mut weight = 0f64;
    let mut score = 0f64;
    let mut collisions = 0;
    let mut temperature = INITIAL_TEMP;
    let mut rng = StdRng::from_entropy();

    while temperature > 0.001 {
        temperature *= MULTIPLIER;

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

        let new_score = new_weight - new_collisions as f64 * coeff;

        if new_score > score || rng.gen::<f64>() < ((new_score - score) / temperature).exp() {
            if answer & th_bit != 0 {
                answer &= !th_bit;
            } else {
                answer |= th_bit;
            }

            score = new_score;
            weight = new_weight;
            collisions = new_collisions;

            if collisions != 0
                || answers.iter().all(|&(_, w)| w > weight)
                || answers.iter().any(|&(a, _)| answer & a == answer)
            {
                continue;
            }

            answers = answers
                .into_iter()
                .filter(|&(a, _)| a & answer != a)
                .collect();

            answers.push((answer, weight));
            answers.sort_by(|(_, w1), (_, w2)| w2.total_cmp(&w1));
            answers.resize(8, Default::default());
        }
    }

    answers.into_iter().take(5).collect()
}

pub fn print_answer(answers: &Vec<(u64, f64)>) {
    let mut output: String = (0..SIZE).map(|i| format!(",TH{}", i + 1)).collect();

    output += ",sum(w)\n";

    for (i, (hypot, weight)) in answers.iter().enumerate() {
        output += &format!("GH{},", i + 1);

        for th in 0..SIZE {
            if hypot & (1u64 << th) != 0 {
                output += "1,";
            } else {
                output += "0,";
            }
        }

        output += &format!("{weight:.4}\n");
    }

    std::fs::write("out.csv", output).unwrap_or_else(|err| {
        eprintln!("error writing to out.csv: {err}");
    });
}

pub fn parse_input(input: &str) -> (CompatibilityMatrix, Vec<f64>) {
    let mut matrix = CompatibilityMatrix::default();

    for (i, line) in input.lines().enumerate().take(SIZE) {
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

    let weights = input
        .lines()
        .last()
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse::<f64>().unwrap())
        .collect();

    (matrix, weights)
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
