use std::{cmp::min, io::{self, Write}};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    Empty,
    Red,
    Yellow
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum State {
    Win,
    None
}


struct Board {
    tiles: [Color; WIDTH * HEIGHT],
    turn: Color
}

impl Board {
    pub fn init() -> Board {
        Board {
            tiles: [Color::Empty; WIDTH * HEIGHT],
            turn: Color::Red
        }
    }
    
    fn check_drop(&self, col: usize, row: usize) -> State {
        let mut num_y = 1;

        for y in row+1..HEIGHT {
            if self.tiles[y * WIDTH + col] == self.turn {
                num_y += 1;
                continue
            }
            break
        }
        for y in (0..row).rev() {
            if self.tiles[y * WIDTH + col] == self.turn {
                num_y += 1;
                continue
            }
            break
        }
        if num_y >= 4 {
            return State::Win;
        }


        let mut num_x = 1;
        for x in col+1..WIDTH {
            if self.tiles[row * WIDTH + x] == self.turn {
                num_x += 1;
                continue
            }
            break
        }
        for x in (0..col).rev() {
            if self.tiles[row * WIDTH + x] == self.turn {
                num_x += 1;
                continue
            }
            break
        }

        if num_x >= 4 {
            return State::Win;
        }

        let mut count = 1;
        for offset in 1..min(WIDTH - col, HEIGHT - row) {
            if self.tiles[(row + offset) * WIDTH + col + offset] == self.turn {
                count += 1;
                continue
            }
            break
        }
        for offset in (1..=min(col, row)).rev() {
            if self.tiles[(row - offset) * WIDTH + col - offset] == self.turn {
                count += 1;
                continue
            }
            break
        }
        
        if count >= 4 {
            return State::Win;
        }

        let mut count = 1;
        for offset in 1..min(col, HEIGHT - row) {
            if self.tiles[(row + offset) * WIDTH + col - offset] == self.turn {
                count += 1;
                continue
            }
            break
        }

        for offset in (1..=min(WIDTH - col, row)).rev() {
            if self.tiles[(row - offset) * WIDTH + col + offset] == self.turn {
                count += 1;
                continue
            }
            break
        }
        
        if count >= 4 {
            return State::Win;
        }


        State::None
    }

    pub fn drop(&mut self, col: usize) -> State {
        for y in 0..HEIGHT {
            if self.tiles[y * WIDTH + col] == Color::Empty {
                self.tiles[y * WIDTH + col] = self.turn;
                let res = self.check_drop(col, y);
                self.turn = match self.turn {
                    Color::Red => Color::Yellow,
                    Color::Yellow => Color::Red,
                    _ => panic!("wtf")
                };
                return res
            }
        }
        State::None
    }

    pub fn print(&self) {
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                print!("{}", match self.tiles[y * WIDTH + x] {
                    Color::Red => " x ",
                    Color::Yellow => " o ",
                    _ => " . "
                });
            }
            println!();
        }
    }
}



fn main() {
    let mut board = Board::init();
    
    loop {
        print!("Enter move ({} turn): ", if board.turn == Color::Yellow { "o" } else { "x" });
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let column: usize = input.trim().parse::<usize>().unwrap();

        if (1..=WIDTH).contains(&column) {
            let state = board.drop(column - 1);
            board.print();
            if state == State::Win {
                println!("Wow congrats you won!");
                break
            }
        }
    }
}
