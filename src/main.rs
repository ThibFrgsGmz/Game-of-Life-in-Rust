// mod display;
// mod grid;
// mod rules;
// mod simulation;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Live,
}

fn main() {
    println!("Hello, world!");

    let width = 10;
    let height = 20;

    let mut array = vec![vec![Cell::Dead; width]; height];

    let mut rng = rand::thread_rng();
    let cell_dist = Uniform::from(0..2);

    for (i, row) in array.iter().enumerate() {
        for (j, _cell) in row.iter().enumerate() {
            let mut _xx = array[i][j];
            _xx = match cell_dist.sample(&mut rng) {
                0 => Cell::Dead,
                _ => Cell::Live,
            };
        }
    }

    for (i, row) in array.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Cell::Dead => println!("Cell at ({}, {}) is Dead", i, j),
                Cell::Live => println!("Cell at ({}, {}) is Alive", i, j),
            }
        }
    }
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
