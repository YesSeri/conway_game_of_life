use std::fmt;
use std::thread::sleep;
use std::time::Duration;
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(y: usize, x: usize) -> Self {
        Point { x: x, y: y }
    }
}
#[derive(Debug, Clone)]
struct Board {
    positions: Vec<Vec<bool>>,
}
impl Board {
    fn tick(&mut self) {
        let mut flips: Vec<Point> = Vec::new();
        for i in 0..self.positions.len() {
            for j in 0..self.positions[i].len() {
                if self.flips_state(i, j) {
                    flips.push(Point::new(j, i));
                }
            }
        }
        for point in flips {
            self.positions[point.y][point.x] = !self.positions[point.y][point.x];
        }
    }
    fn flips_state(&self, i: usize, j: usize) -> bool {
        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Any live cell with two or three live neighbours lives on to the next generation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        // This can be shortened to the following:
        // Live cell with less than two or more than three neighbours flips state. Dead cell with exactly three neighbours flip state.
        // All other cells stays the same.
        let sum = self.sum_neighbours(i, j);
        let square = self.positions[i][j];
        if (square && (sum < 2 || sum > 3)) || (!square && sum == 3) {
            return true;
        }
        false
    }
    fn sum_neighbours(&self, i: usize, j: usize) -> u8 {
        let mut squares_to_check = vec![true; 8];
        // Dont check the row to the left. We are at left most row.
        if i == 0 {
            squares_to_check[6] = false;
            squares_to_check[7] = false;
            squares_to_check[0] = false;
        }
        // Dont check row above. We are at top row.
        if j == 0 {
            squares_to_check[0] = false;
            squares_to_check[1] = false;
            squares_to_check[2] = false;
        }
        // Dont check the row to the right. We are at right most row.
        if i == self.positions.len() - 1 {
            squares_to_check[2] = false;
            squares_to_check[3] = false;
            squares_to_check[4] = false;
        }
        // Dont check row below. We are at bottom row.
        if j == self.positions.get(i).unwrap().len() - 1 {
            squares_to_check[4] = false;
            squares_to_check[5] = false;
            squares_to_check[6] = false;
        }
        let mut sum: u8 = 0;
        for (idx, square) in squares_to_check.into_iter().enumerate() {
            if idx == 0 && square {
                sum += self.get_value(i - 1, j - 1);
            } else if idx == 1 && square {
                sum += self.get_value(i, j - 1);
            } else if idx == 2 && square {
                sum += self.get_value(i + 1, j - 1);
            } else if idx == 3 && square {
                sum += self.get_value(i + 1, j);
            } else if idx == 4 && square {
                sum += self.get_value(i + 1, j + 1);
            } else if idx == 5 && square {
                sum += self.get_value(i, j + 1);
            } else if idx == 6 && square {
                sum += self.get_value(i - 1, j + 1);
            } else if idx == 7 && square {
                sum += self.get_value(i - 1, j);
            }
        }
        return sum;
    }
    // i is row, j is col
    fn get_value(&self, i: usize, j: usize) -> u8 {
        match self.positions.get(i) {
            Some(row) => match row.get(j) {
                Some(square) => {
                    if *square == true {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            },
            None => 0,
        }
    }
}
impl From<Vec<Vec<bool>>> for Board {
    fn from(v: Vec<Vec<bool>>) -> Self {
        Board { positions: v }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, v) in self.positions.iter().enumerate() {
            if i > 0 {
                s += "\n";
            }
            for p in v {
                if *p {
                    s += " * "
                } else {
                    s += " - "
                }
            }
        }
        write!(f, "{}", s)
    }
}
struct Game {
    board: Board,
    sleep_time: u32,
}
impl Game {
    fn new(time: u32, size: usize) -> Self {
        let vector = vec![vec![false; size]; size];
        // let flipper_vector = vec![
        //     vec![false, false, false],
        //     vec![true, true, true],
        //     vec![false, false, false],
        // ];
        Game {
            board: Board::from(vector),
            sleep_time: time,
        }
    }
    fn start_game(&mut self) {
        loop {
            sleep(Duration::from_millis(self.sleep_time as u64));
            print!("{}[2J", 27 as char);
            println!("{}", self.board);
            self.board.tick();
        }
    }
    fn change_value(&mut self, i: usize, j: usize) {
        self.board.positions[i][j] = !self.board.positions[i][j]
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.board.fmt(f)
    }
}

use std::{env, io};
fn main() {
    let args: Vec<String> = env::args().collect();
    let time = args.get(1).unwrap();
    let size = args.get(2).unwrap();
    let mut game = Game::new(time.parse().unwrap(), size.parse().unwrap());
    loop {
        println!("{}", game);
        println!("Enter position to flip. They are zero indexed. Press c to continue");
        println!("Col: ");
        let col = read_value();
        println!("Row: ");
        if col == "c" {
            break;
        }
        let row = read_value();
        if row == "c" {
            break;
        }
        let col = match col.parse::<usize>() {
            Ok(c) => c,
            Err(_) => continue,
        };
        let row = match row.parse::<usize>() {
            Ok(c) => c,
            Err(_) => continue,
        };
        dbg!(col, row);
        game.change_value(col, row);
    }
    game.start_game();
}

fn read_value() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to readline");
    s.trim().to_string()
}
