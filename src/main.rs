use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::io::stdout;
use structopt::StructOpt;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridCell {
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

/// Entrypoint of the program. It sets up command-line arguments, creates and initializes
/// a grid of cells, and then prints the grid to the terminal. The grid size and initial
/// number of live cells can be specified by the user via command-line arguments.
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

/// Creates a new 2D array of dead cells with the specified width and height.
///
/// This function uses the `vec!` macro to create a vector of vectors, with each inner
/// vector representing a row of cells in the grid. All cells are initially dead.
///
/// # Arguments
///
/// * `width` - The number of cells in each row (i.e., the number of columns in the grid).
/// * `height` - The number of rows in the grid.
fn create_array(width: usize, height: usize) -> Vec<Vec<GridCell>> {
    vec![vec![GridCell::Dead; width]; height]
}

/// Randomly sets a specified number of cells in the given array to be live.
///
/// This function first creates a list of all possible indices into the array, then
/// shuffles that list to get a random order. It sets the first `live` cells in the
/// shuffled list to be live.
///
/// The use of a single, linear list of indices allows for simple and efficient randomization
/// and avoids the need to manually handle 2D indexing.
///
/// # Arguments
///
/// * `array` - The 2D array of cells to initialize.
/// * `width` - The number of cells in each row (i.e., the number of columns in the grid).
/// * `height` - The number of rows in the grid.
/// * `live` - The number of cells to initialize as live.
fn initialize_array(array: &mut [Vec<GridCell>], width: usize, height: usize, live: usize) {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut indices: Vec<_> = (0..width * height).collect();
    indices.shuffle(&mut rng);

    indices.iter().take(live).for_each(|&i| {
        array[i / width][i % width] = GridCell::Live;
    });
}

/// Prints the given array of cells to the terminal using the TUI library.
///
/// This function first transforms the 2D array into a list of Rows, with each row
/// containing styled Cells. Live cells are represented as white squares, while dead
/// cells are represented as black.
///
/// After creating the Rows, it uses TUI to create a Table widget and adds the rows to it.
/// The table is styled with borders and a title, and all columns are set to a width of 1
/// to provide a compact, grid-like appearance.
///
/// Finally, the table is rendered to the terminal inside a layout chunk that takes up the
/// full size of the terminal.
///
/// # Arguments
///
/// * `array` - The 2D array of cells to print.
fn print_array(array: &[Vec<GridCell>]) {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    let rows = array
        .iter()
        .map(|row| {
            let cells = row.iter().map(|cell| match cell {
                GridCell::Live => {
                    tui::widgets::Cell::from(Span::styled("■", Style::default().fg(Color::White)))
                }
                GridCell::Dead => tui::widgets::Cell::from(Span::styled(
                    " ",
                    Style::default().fg(Color::White).bg(Color::Black),
                )),
            });
            Row::new(cells)
        })
        .collect::<Vec<Row>>();

    let widths = vec![Constraint::Length(1); array[0].len()];

    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let table = Table::new(rows)
                .block(Block::default().borders(Borders::ALL).title("Cell Grid"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .widths(&widths);

            f.render_widget(table, chunks[0]);
        })
        .unwrap();
}
