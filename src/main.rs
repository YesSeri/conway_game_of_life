mod board;
mod game;
mod point;
use game::Game;

/// # Conway's Game of Life
/// This is an implementation written by Henrik Zenkert
/// ## To run
/// Use `cargo run -- --time 100 --size 20` to run game at 10 fps and a 20x20 grid.
/// To run the executable run `/path/to/executable/conway-hank --time 80 --size 20`.
/// ## The rules
/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
/// This can be shortened to the following:
/// - Live cell with less than two or more than three neighbours flips state. Dead cell with exactly three neighbours flip state.
/// - All other cells stays the same.
///
///
use clap::Parser;

/// # Conway's Game of Life
/// This is an implementation written by Henrik Zenkert
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time in ms between each tick
    #[arg(short, long)]
    time: usize,

    /// Size of the grid, n x n, where --size n
    #[arg(short, long)]
    size: usize,
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new(args.time, args.size);
    game.setup_board();
    game.start_game();
}
