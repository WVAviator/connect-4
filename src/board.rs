// Cells are identified as A1 (top-right) to G6 (bottom-right)
// Isolating the most significant bit is more difficult than the least significant bit, so the
// bitboard repr uses A1 as lsb 8, and G6 as the msb 48. The remaining 16 bits are unused.
// The first row is filled with arbitrary pieces to facilitate finding the lsb of the bottom row.

use anyhow::{anyhow, bail};
use arrayvec::ArrayVec;
use colored::Colorize;
use std::fmt;

use crate::constants::{BOARD_MASK, EMPTY_BOARD, FILE, GAME_MASK, ROW};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Board {
    red: u64,
    yellow: u64,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    Red = 0,
    Yellow = 1,
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
        let cell = (file >> 7) & !self.all();

        self.red |= cell * ((color as u64) ^ 1);
        self.yellow |= cell * (color as u64);
    }

    pub fn remove(&mut self, file: usize) {
        let file = FILE[file] & self.all() & GAME_MASK;
        let lsb = file & (!file + 1);
        self.red &= !lsb;
        self.yellow &= !lsb;
    }

    pub fn evaluate(&self) -> i32 {
        let mut score: i32 = 0;

        let red_pieces = self.red & GAME_MASK;
        let yellow_pieces = self.yellow & GAME_MASK;
        let empty = self.empty();

        // This starts with all the red pieces, and then performs all subsequent scanning with
        // empty cells - indicating the "potential" for a connect-4. The result is ANDed with the
        // original piece positions to count every piece that is potentially part of a connect-4.
        // Ideally this should not score piece placements that can never acheive a connect-4.

        let mut horizontal_check = red_pieces;
        let mut vertical_check = red_pieces;
        let mut diagonal_check = red_pieces;
        let mut antidiagonal_check = red_pieces;
        for _ in 0..3 {
            horizontal_check = ((horizontal_check & !FILE[0]) >> 1) & (red_pieces | empty);
            vertical_check = ((vertical_check & !ROW[5]) >> 7) & (red_pieces | empty);
            diagonal_check = ((diagonal_check & !ROW[5] & !FILE[6]) >> 6) & (red_pieces | empty);
            antidiagonal_check =
                ((antidiagonal_check & !ROW[5] & !FILE[0]) >> 8) & (red_pieces | empty);
        }

        score += (horizontal_check & red_pieces).count_ones() as i32;
        score += (vertical_check & red_pieces).count_ones() as i32;
        score += (diagonal_check & red_pieces).count_ones() as i32;
        score += (antidiagonal_check & red_pieces).count_ones() as i32;

        let mut horizontal_check = yellow_pieces;
        let mut vertical_check = yellow_pieces;
        let mut diagonal_check = yellow_pieces;
        let mut antidiagonal_check = yellow_pieces;
        for _ in 0..3 {
            horizontal_check = ((horizontal_check & !FILE[0]) >> 1) & (yellow_pieces | empty);
            vertical_check = ((vertical_check & !ROW[5]) >> 7) & (yellow_pieces | empty);
            diagonal_check = ((diagonal_check & !ROW[5] & !FILE[6]) >> 6) & (yellow_pieces | empty);
            antidiagonal_check =
                ((antidiagonal_check & !ROW[5] & !FILE[0]) >> 8) & (yellow_pieces | empty);
        }

        score -= (horizontal_check & yellow_pieces).count_ones() as i32;
        score -= (vertical_check & yellow_pieces).count_ones() as i32;
        score -= (diagonal_check & yellow_pieces).count_ones() as i32;
        score -= (antidiagonal_check & yellow_pieces).count_ones() as i32;

        // Separate scoring for actual connect-4

        let mut horizontal_check = red_pieces;
        let mut vertical_check = red_pieces;
        let mut diagonal_check = red_pieces;
        let mut antidiagonal_check = red_pieces;
        for _ in 0..3 {
            horizontal_check = ((horizontal_check & !FILE[0]) >> 1) & red_pieces;
            vertical_check = ((vertical_check & !ROW[5]) >> 7) & red_pieces;
            diagonal_check = ((diagonal_check & !ROW[5] & !FILE[6]) >> 6) & red_pieces;
            antidiagonal_check = ((antidiagonal_check & !ROW[5] & !FILE[0]) >> 8) & red_pieces;
        }

        score += (horizontal_check & red_pieces).count_ones() as i32 * 42;
        score += (vertical_check & red_pieces).count_ones() as i32 * 42;
        score += (diagonal_check & red_pieces).count_ones() as i32 * 42;
        score += (antidiagonal_check & red_pieces).count_ones() as i32 * 42;

        let mut horizontal_check = yellow_pieces;
        let mut vertical_check = yellow_pieces;
        let mut diagonal_check = yellow_pieces;
        let mut antidiagonal_check = yellow_pieces;
        for _ in 0..3 {
            horizontal_check = ((horizontal_check & !FILE[0]) >> 1) & yellow_pieces;
            vertical_check = ((vertical_check & !ROW[5]) >> 7) & yellow_pieces;
            diagonal_check = ((diagonal_check & !ROW[5] & !FILE[6]) >> 6) & yellow_pieces;
            antidiagonal_check = ((antidiagonal_check & !ROW[5] & !FILE[0]) >> 8) & yellow_pieces;
        }

        score -= (horizontal_check & yellow_pieces).count_ones() as i32 * 42;
        score -= (vertical_check & yellow_pieces).count_ones() as i32 * 42;
        score -= (diagonal_check & yellow_pieces).count_ones() as i32 * 42;
        score -= (antidiagonal_check & yellow_pieces).count_ones() as i32 * 42;

        score
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

    pub fn legal_files(&self) -> ArrayVec<usize, 7> {
        let mut legal_files = ArrayVec::new();
        let mut top_row = self.empty() & ROW[5];
        while top_row != 0 {
            let lsb = top_row & (!top_row + 1);
            legal_files.push(lsb.trailing_zeros() as usize);
            top_row &= top_row - 1;
        }

        legal_files
    }
}

impl Color {
    pub fn other(&self) -> Self {
        match self {
            Color::Red => Color::Yellow,
            Color::Yellow => Color::Red,
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "{}", "R".red()),
            Color::Yellow => write!(f, "{}", "Y".yellow()),
        }
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
