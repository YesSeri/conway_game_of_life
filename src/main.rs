mod board;
mod game;
mod point;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.setup_board();
    game.start_game();
}
