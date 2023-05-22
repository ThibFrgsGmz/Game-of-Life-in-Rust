mod display;
mod grid;
mod rules;
mod simulation;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    if let Some(c) = generate_random_char() {
        println!("char: {}", c);
    }

    let _y = generate_random_float();

    let _nums = generate_random_array();
}

fn generate_random_char() -> Option<char> {
    if rand::random() {
        // generates a boolean
        Some(rand::random::<char>()) // generates a random character
    } else {
        None
    }
}

fn generate_random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen() // generates a float between 0 and 1
}

fn generate_random_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_char() {
        let c = generate_random_char();
        assert!(c.is_none() || c.unwrap().is_alphabetic());
    }

    #[test]
    fn test_generate_random_float() {
        let f = generate_random_float();
        assert!(f >= 0.0 && f <= 1.0);
    }

    #[test]
    fn test_generate_random_array() {
        let nums = generate_random_array();
        assert_eq!(nums.len(), 99);
        for i in 1..100 {
            assert!(nums.contains(&i));
        }
    }
}
