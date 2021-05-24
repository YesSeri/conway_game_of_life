use crate::point::Point;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Board {
    pub positions: Vec<Vec<bool>>,
    iter: u32,
}
impl Board {
    pub fn tick(&mut self) {
        self.print_tick_iter();
        self.iter += 1;
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
    fn print_tick_iter(&self) {
        println!("Tick: {}", self.iter);
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

        // Clippy prefers this rather than the more readable: if (square && (sum < 2 || sum > 3)) || (!square && sum == 3)
        if (square && !(2..=3).contains(&sum)) || (!square && sum == 3) {
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
            } else if sum > 3 {
                // All sums larger than 3 have the same effect. This is for a bit of efficency.
                return sum;
            }
        }
        sum
    }
    // i is row, j is col
    fn get_value(&self, i: usize, j: usize) -> u8 {
        // The none values are if the i or j is out of bounds. If out of bounds I just want to count it as not an active neighbour.
        match self.positions.get(i) {
            Some(row) => match row.get(j) {
                Some(square) => {
                    if *square {
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
    pub fn print_with_idx(&self) {
        let mut s = String::from("   ");
        for i in 0..self.positions.get(0).unwrap().len() {
            s += " ";
            s += &i.to_string();
            s += " ";
        }
        s += "\n";
        for (i, v) in self.positions.iter().enumerate() {
            if i != 0 {
                s += "\n";
            }
            s += " ";
            s += &i.to_string();
            s += " ";
            for p in v {
                if *p {
                    s += " X "
                } else {
                    s += " - "
                }
            }
        }
        println!("{}", s);
    }
}
impl From<Vec<Vec<bool>>> for Board {
    fn from(v: Vec<Vec<bool>>) -> Self {
        Board {
            positions: v,
            iter: 0,
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, v) in self.positions.iter().enumerate() {
            if i != 0 {
                s += "\n";
            }
            for p in v {
                if *p {
                    s += " X "
                } else {
                    s += " - "
                }
            }
        }
        write!(f, "{}", s)
    }
}
