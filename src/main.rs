mod board;
mod point;
mod game;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.setup_board();
    game.start_game();
}
