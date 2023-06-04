use rand::distributions::{Distribution, Uniform};
// use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Live,
}

pub fn calculate(op: char, a: i64, b: i64) -> Option<i64> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b != 0 {
                Some(a / b)
            } else {
                None
            }
        }
        _ => None,
    }
}
fn main() {
    println!("Hello, world!");

    // let op = '+';
    // let a = 5;
    // let b = 3;
    // match calculate(op, a, b) {
    //     Some(result) => println!("Result: {}", result),
    //     None => println!("Error: invalid operation or division by zero"),
    // }

    let width = 10;
    let height = 20;
    let mut array = vec![vec![Cell::Dead; width]; height];

    let mut rng = rand::thread_rng();
    let cell_dist = Uniform::from(0..2);

    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            array[i][j] = match cell_dist.sample(&mut rng) {
                0 => Cell::Dead,
                _ => Cell::Live,
            };
            println!("Cell at ({}, {}) is {:?}", i, j, array[i][j]);
        });
    });

    array.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| match cell {
            Cell::Dead => println!("Cell at ({}, {}) is Dead", i, j),
            _ => println!("Cell at ({}, {}) is Alive", i, j),
        });
    });
}

// fn generate_random_float() -> f64 {
//     let mut rng = rand::thread_rng();
//     rng.gen() // generates a float between 0 and 1
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_generate_random_float() {
//         let f = generate_random_float();
//         assert!((0.0..1.0).contains(&f));
//     }
// }
