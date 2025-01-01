use std::io::{BufRead, BufReader, Stdin};

use colored::Colorize;
use rand::random;

use crate::{
    board::{Board, Color},
    minimax::Minimax,
};

pub struct Repl {
    board: Board,
    turn: Color,
    player: Color,
    reader: BufReader<Stdin>,
}

impl Default for Repl {
    fn default() -> Self {
        Repl::new()
    }
}

impl Repl {
    pub fn new() -> Self {
        let stdin = std::io::stdin();
        let reader = BufReader::new(stdin);

        let turn = match random::<bool>() {
            true => Color::Red,
            false => Color::Yellow,
        };

        Repl {
            board: Board::new(),
            turn,
            player: Color::Yellow,
            reader,
        }
    }

    pub fn start(mut self) {
        self.choose_color();

        loop {
            println!("{}", self.board);
            println!("Turn: {}", self.turn);

            match self.turn == self.player {
                true => {
                    println!("Enter file (1-7) to play: ");

                    let buffer = self.read_input();

                    match buffer.as_str().trim_end() {
                        "newgame" | "n" => self.board = Board::new(),
                        "quit" | "q" => break,
                        file => {
                            if let Ok(file) = file.trim_ascii().parse::<usize>() {
                                if !(0..7).contains(&file) {
                                    println!("Bad file. Please enter 0-6.");
                                    continue;
                                }
                                self.insert_file(file);
                            } else {
                                println!("Unknown input.");
                            }
                        }
                    }
                }
                false => {
                    println!("Computer is thinking...");
                    let minimax = Minimax::new(&self.board, self.player.other(), 10);
                    let file = minimax.best_move();
                    self.insert_file(file);
                }
            }
        }
    }

    fn choose_color(&mut self) {
        println!("Choose your color ({}/{}): ", "Y".yellow(), "R".red());

        loop {
            let buffer = self.read_input();
            match buffer.as_str().trim_end() {
                c if c.starts_with('y') || c.starts_with('Y') => {
                    self.player = Color::Yellow;
                    break;
                }
                c if c.starts_with('r') || c.starts_with('R') => {
                    self.player = Color::Red;
                    break;
                }
                _ => println!(
                    "Invalid color. Please type '{}' or '{}':",
                    "Y".yellow(),
                    "R".red()
                ),
            }
        }
    }

    fn game_over(&self) {
        println!("{}", self.board);
        match self.turn {
            Color::Red => println!("{}", "WINNER!!!".red()),
            Color::Yellow => println!("{}", "WINNER!!!".yellow()),
        }
    }

    fn play_again(&mut self) -> bool {
        println!("\nStart new game (y/n)? ");

        let buffer = self.read_input();

        match buffer.as_str().trim_end() {
            "n" | "no" => return false,
            _ => self.board = Board::new(),
        }

        true
    }

    fn read_input(&mut self) -> String {
        let mut buffer = String::new();

        if self.reader.read_line(&mut buffer).is_err() {
            panic!("info string \"failed to initialize stdin\" ");
        }

        buffer
    }

    fn insert_file(&mut self, file: usize) {
        self.board.insert(file, self.turn);
        if self.board.has_connect_4(self.turn) {
            self.game_over();
            if !self.play_again() {
                std::process::exit(0);
            }
        }
        self.turn = match self.turn {
            Color::Yellow => Color::Red,
            Color::Red => Color::Yellow,
        };
    }
}
