use crate::board::Board;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use std::{env, io};
pub struct Game {
    board: Board,
    sleep_time: u32,
}
impl Game {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let time = args
            .get(1)
            .unwrap_or(&String::from("1000"))
            .parse()
            .unwrap();
        let size = args.get(2).unwrap_or(&String::from("10")).parse().unwrap();
        let vector = vec![vec![false; size]; size];
        Game {
            board: Board::from(vector),
            sleep_time: time,
        }
    }
    pub fn start_game(&mut self) {
        loop {
            print!("{}[2J", 27 as char);
            println!("{}", self.board);
            self.board.tick();
            sleep(Duration::from_millis(self.sleep_time as u64));
        }
    }
    fn change_value(&mut self, i: usize, j: usize) {
        // self.board.positions[i][j] = !self.board.positions[i][j]
        let row = match self.board.positions.get(i) {
            Some(r) => r,
            None => return,
        };
        match row.get(j) {
            Some(_) => true,
            None => return,
        };
        self.board.positions[i][j] = !self.board.positions[i][j];
    }
    pub fn setup_board(&mut self) {
        loop {
            self.board.print_with_idx();
            println!("Enter position to flip. They are zero indexed. Press r to randomize. Press c to continue");
            println!("Col: ");
            let col = Game::read_value();
            println!("Row: ");
            if col == "c" {
                break;
            } else if col == "r" {
                self.random_setup();
            }
            let col = match col.parse::<usize>() {
                Ok(c) => c,
                Err(_) => continue,
            };
            let row = Game::read_value();
            if row == "c" {
                break;
            } else if row == "r" {
                self.random_setup();
            }
            let row = match row.parse::<usize>() {
                Ok(c) => c,
                Err(_) => continue,
            };
            self.change_value(col, row);
        }
    }
    fn read_value() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to readline");
        s.trim().to_string()
    }
    fn random_setup(&mut self) {
        for i in 0..self.board.positions.len() {
            for j in 0..self.board.positions[i].len() {
                self.board.positions[i][j] = rand::random();
            }
        }
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.board.fmt(f)
    }
}
