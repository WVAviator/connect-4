
// Cells are identified as A1 (top-right) to G6 (bottom-right)
// Isolating the most significant bit is more difficult than the least significant bit, so the
// bitboard repr uses A1 as lsb 8, and G6 as the msb 48. The remaining 16 bits are unused.
// The first row is filled with arbitrary pieces to facilitate finding the lsb of the bottom row.

use std::fmt;
use colored::Colorize;

use crate::constants::{EMPTY_BOARD, FILE};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    red: u64,
    yellow: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Yellow,
}

impl Board {
    pub fn new() -> Self {
        Board {
            red: EMPTY_BOARD,
            yellow: EMPTY_BOARD,
        }
    }

    #[inline(always)]
    pub fn all(&self) -> u64 {
        self.red | self.yellow
    }

    pub fn insert(&mut self, file: usize, color: Color) {
        let file = FILE[file] & self.all();
        let lsb = file & (!file + 1);
        let cell = lsb >> 7;

        match color {
            Color::Red => {
                self.red |= cell;
            }
            Color::Yellow => self.yellow |= cell,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..42 {
            if i % 7 == 0 {
                write!(f, "{}", "|".blue())?;
            }
            if self.red & (1 << i) != 0 {
                write!(f, "{}", " ⬤ ".red())?;
            } else if self.yellow & (1 << i) != 0 {
                write!(f, "{}", " ⬤ ".yellow())?;
            } else {
                write!(f, "{}", " ◯ ".blue())?;
            }
            if i % 7 == 6 {
                writeln!(f, "{}", "|".blue())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inserts_into_empty_board() {
        let mut board = Board::new();

        board.insert(2, Color::Yellow);
        assert_eq!(board.yellow, 0x0001FC2000000000);

        board.insert(2, Color::Red);
        assert_eq!(board.red, 0x0001FC0040000000);

        println!("{}", board);
    }

    #[test]
    fn inserts_into_first_last_files() {
        let mut board = Board::new();

        board.insert(6, Color::Yellow);
        assert_eq!(board.yellow, 0x0001FE0000000000);

        board.insert(0, Color::Red);
        assert_eq!(board.red, 0x0001FC0800000000);

        println!("{}", board);
    }
}
