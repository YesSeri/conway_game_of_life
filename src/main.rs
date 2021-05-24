mod board;
mod game;
mod point;
use game::Game;

/// # Conway's Game of Life
/// This is an implementation written by Henrik Zenkert
/// ## To run
/// Use `cargo run 1000 50` to run game at 1 fps and a size of 50.
/// ## The rules
/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
/// This can be shortened to the following:
/// - Live cell with less than two or more than three neighbours flips state. Dead cell with exactly three neighbours flip state.
/// - All other cells stays the same.

fn main() {
    let mut game = Game::new();
    game.setup_board();
    game.start_game();
}
