use std::io::{BufRead, BufReader};

use colored::Colorize;

use crate::board::{Board, Color};

pub struct Repl {
    board: Board,
    turn: Color,
}

impl Default for Repl {
    fn default() -> Self {
        Repl::new()
    }
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            board: Board::new(),
            turn: Color::Yellow,
        }
    }

    pub fn start(mut self) {
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin);

        loop {
            println!("{}", self.board);
            println!("Turn: {}", self.turn);
            println!("Enter file (1-7) to play: ");

            let mut buffer = String::new();
            if reader.read_line(&mut buffer).is_err() {
                panic!("info string \"failed to initialize stdin\" ");
            }

            match buffer.as_str().trim_end() {
                "newgame" | "n" => self.board = Board::new(),
                "quit" | "q" => break,
                file => {
                    if let Ok(file) = file.trim_ascii().parse::<usize>() {
                        if !(0..7).contains(&file) {
                            println!("Bad file. Please enter 0-6.");
                        }
                        self.board.insert(file, self.turn);
                        if self.board.has_connect_4(self.turn) {
                            println!("{}", self.board);
                            match self.turn {
                                Color::Red => println!("{}", "WINNER!!!".red()),
                                Color::Yellow => println!("{}", "WINNER!!!".yellow()),
                            }
                            println!("\nStart new game (y/n)? ");
                            let mut buffer = String::new();
                            if reader.read_line(&mut buffer).is_err() {
                                panic!("info string \"failed to initialize stdin\" ");
                            }
                            match buffer.as_str().trim_end() {
                                "n" | "no" => break,
                                _ => self.board = Board::new(),
                            }
                        }
                        self.turn = match self.turn {
                            Color::Yellow => Color::Red,
                            Color::Red => Color::Yellow,
                        };
                    } else {
                        println!("Unknown input.");
                    }
                }
            }
        }
    }
}
