use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    if rand::random() { // generates a boolean
        println!("char: {}", rand::random::<char>());
    }

    let mut rng = rand::thread_rng();
    let _y: f64 = rng.gen(); // generates a float between 0 and 1

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
}
