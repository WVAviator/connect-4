// Cells are identified as A1 (top-right) to G6 (bottom-right)
// Isolating the most significant bit is more difficult than the least significant bit, so the
// bitboard repr uses A1 as lsb 8, and G6 as the msb 48. The remaining 16 bits are unused.
// The first row is filled with arbitrary pieces to facilitate finding the lsb of the bottom row.

use anyhow::{anyhow, bail};
use colored::Colorize;
use std::fmt;

use crate::constants::{BOARD_MASK, EMPTY_BOARD, FILE, GAME_MASK, ROW};

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

    pub fn from_notation(notation: &str) -> Result<Self, anyhow::Error> {
        // "7/7/7/7/5r1/4yr1/4ryy"

        let mut board = Board::new();
        let mut index = 0;

        for ch in notation.chars() {
            if index >= 42 {
                bail!(
                    "invalid row count in notation. expected 6, got {}: {}",
                    notation.split("/").count(),
                    notation
                );
            }
            match ch {
                c if c.is_ascii_digit() => {
                    index += c
                        .to_digit(10)
                        .ok_or(anyhow!("could not parse digit: {}", c))?
                }
                'r' => {
                    board.red |= 1 << index;
                    index += 1;
                }
                'y' => {
                    board.yellow |= 1 << index;
                    index += 1;
                }
                '/' => {
                    if index % 7 != 0 {
                        bail!(
                            "invalid row count in notation ending at index {}: {}",
                            index,
                            notation
                        );
                    }
                }
                c => bail!("invalid character in notation: {}", c),
            }
        }

        Ok(board)
    }

    #[inline(always)]
    pub fn all(&self) -> u64 {
        self.red | self.yellow
    }

    #[inline(always)]
    pub fn empty(&self) -> u64 {
        !(self.red | self.yellow) & BOARD_MASK
    }

    pub fn insert(&mut self, file: usize, color: Color) {
        let file = FILE[file] & self.all();
        let lsb = file & (!file + 1);
        let cell = (lsb >> 7) & BOARD_MASK;

        match color {
            Color::Red => {
                self.red |= cell;
            }
            Color::Yellow => self.yellow |= cell,
        }
    }

    pub fn has_connect_4(&self, color: Color) -> bool {
        let pieces = match color {
            Color::Yellow => self.yellow,
            Color::Red => self.red,
        } & GAME_MASK;

        let mut horizontal_check = pieces;
        let mut vertical_check = pieces;
        let mut diagonal_check = pieces;
        let mut antidiagonal_check = pieces;
        for _ in 0..3 {
            horizontal_check = ((horizontal_check & !FILE[0]) >> 1) & pieces;
            vertical_check = ((vertical_check & !ROW[5]) >> 7) & pieces;
            diagonal_check = ((diagonal_check & !ROW[5] & !FILE[6]) >> 6) & pieces;
            antidiagonal_check = ((antidiagonal_check & !ROW[5] & !FILE[0]) >> 8) & pieces;
        }

        horizontal_check | vertical_check | diagonal_check | antidiagonal_check != 0
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

    #[test]
    fn cannot_insert_past_sixth_row() {
        let mut board = Board::from_notation("2r4/2y4/2r4/2y4/2r4/2y4").unwrap();

        board.insert(2, Color::Yellow);
        assert_eq!(board.yellow, 0x0001FC2000800200);

        board.insert(2, Color::Red);
        assert_eq!(board.red, 0x0001FC0040010004);

        println!("{}", board);
    }

    #[test]
    fn finds_horizontal_connect_4() {
        let board = Board::from_notation("7/7/7/7/2yyy2/2rrrr1").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Red));
        assert!(!board.has_connect_4(Color::Yellow));
    }

    #[test]
    fn finds_horizontal_connect_4_edge() {
        let board = Board::from_notation("7/7/7/7/yyy4/rrrr3").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Red));
        assert!(!board.has_connect_4(Color::Yellow));
    }

    #[test]
    fn finds_vertical_connect_4() {
        let board = Board::from_notation("7/7/6y/r5y/r5y/r5y").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Yellow));
        assert!(!board.has_connect_4(Color::Red));
    }

    #[test]
    fn finds_vertical_connect_4_edge() {
        let board = Board::from_notation("6y/6y/6y/6y/6r/4rrr").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Yellow));
        assert!(!board.has_connect_4(Color::Red));
    }

    #[test]
    fn finds_diagonal_connect_4() {
        let board = Board::from_notation("7/7/3y3/2y3r/1y4r/y5r").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Yellow));
        assert!(!board.has_connect_4(Color::Red));
    }

    #[test]
    fn finds_diagonal_connect_4_edge() {
        let board = Board::from_notation("6y/5yr/4yr1/3yr2/7/7").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Yellow));
        assert!(!board.has_connect_4(Color::Red));
    }

    #[test]
    fn finds_antidiagonal_connect_4() {
        let board = Board::from_notation("r6/yr5/1yr4/2yr3/7/7").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Red));
        assert!(!board.has_connect_4(Color::Yellow));
    }
    
    #[test]
    fn finds_antidiagonal_connect_4_edge() {
        let board = Board::from_notation("7/3yr2/4yr1/5yr/6y/7").unwrap();

        println!("{}", board);

        assert!(board.has_connect_4(Color::Yellow));
        assert!(!board.has_connect_4(Color::Red));
    }

}
