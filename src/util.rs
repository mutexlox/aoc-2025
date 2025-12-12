use std::env;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_lines() -> std::io::Lines<BufReader<File>> {
    let arg = env::args().nth(1).expect("Need file to read");
    let path = Path::new(&arg);
    let file = File::open(path).unwrap_or_else(|_| panic!("could not open {}", path.display()));
    BufReader::new(file).lines()
}

pub fn get_all_input() -> String {
    let arg = env::args().nth(1).expect("Need file to read");
    read_to_string(&arg).unwrap_or_else(|_| panic!("Could not read {}", arg))
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    DownLeft,
    Down,
    DownRight,
    Left,
    Right,
}
impl Direction {
    pub fn neighbor(
        self,
        point: (usize, usize),
        max_x: usize,
        max_y: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::UpLeft => {
                if point.0 == 0 || point.1 == 0 {
                    None
                } else {
                    Some((point.0 - 1, point.1 - 1))
                }
            }
            Direction::Up => {
                if point.0 == 0 {
                    None
                } else {
                    Some((point.0 - 1, point.1))
                }
            }
            Direction::UpRight => {
                if point.0 == 0 || point.1 == max_y - 1 {
                    None
                } else {
                    Some((point.0 - 1, point.1 + 1))
                }
            }
            Direction::DownLeft => {
                if point.0 == max_x - 1 || point.1 == 0 {
                    None
                } else {
                    Some((point.0 + 1, point.1 - 1))
                }
            }
            Direction::Down => {
                if point.0 == max_x - 1 {
                    None
                } else {
                    Some((point.0 + 1, point.1))
                }
            }
            Direction::DownRight => {
                if point.0 == max_x - 1 || point.1 == max_y - 1 {
                    None
                } else {
                    Some((point.0 + 1, point.1 + 1))
                }
            }
            Direction::Left => {
                if point.1 == 0 {
                    None
                } else {
                    Some((point.0, point.1 - 1))
                }
            }
            Direction::Right => {
                if point.1 == max_y - 1 {
                    None
                } else {
                    Some((point.0, point.1 + 1))
                }
            }
        }
    }

    pub const fn directions() -> [Self; 8] {
        [
            Direction::UpLeft,
            Direction::Up,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::Down,
            Direction::DownRight,
            Direction::Left,
            Direction::Right,
        ]
    }
}
