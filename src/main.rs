use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use structopt::StructOpt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Dead,
    Live,
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cell_generator",
    about = "A cell generator with random initialization."
)]
pub struct Opt {
    /// Width of the cell grid
    #[structopt(short = "w", long = "width", default_value = "10")]
    width: usize,

    /// Height of the cell grid
    #[structopt(short = "h", long = "height", default_value = "20")]
    height: usize,

    /// Number of cells to randomly initialize as Live
    #[structopt(short = "n", long = "live", default_value = "10")]
    live: usize,
}

fn main() {
    let opt = Opt::from_args();

    println!(
        "Creating a {} x {} grid with {} live cells...",
        opt.width, opt.height, opt.live
    );

    let mut array = create_array(opt.width, opt.height);
    initialize_array(&mut array, opt.width, opt.height, opt.live);
    print_array(&array);
}

fn create_array(width: usize, height: usize) -> Vec<Vec<Cell>> {
    vec![vec![Cell::Dead; width]; height]
}

fn initialize_array(array: &mut [Vec<Cell>], width: usize, height: usize, live: usize) {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut indices: Vec<_> = (0..width * height).collect();
    indices.shuffle(&mut rng);

    indices.iter().take(live).for_each(|&i| {
        array[i / width][i % width] = Cell::Live;
    });
}

fn print_array(array: &[Vec<Cell>]) {
    for (i, row) in array.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            println!("Cell at ({}, {}) is {:?}", i, j, cell);
        }
    }
}
