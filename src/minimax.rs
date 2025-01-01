use std::cmp;

use crate::board::{Board, Color};

pub struct Minimax<'a> {
    board: &'a Board,
    color: Color,
    depth: usize,
}

impl<'a> Minimax<'a> {
    pub fn new(board: &'a Board, color: Color, depth: usize) -> Self {
        Minimax {
            board,
            color,
            depth,
        }
    }

    pub fn best_move(&self) -> usize {
        let files = self.board.legal_files();
        let mut evaluations: Vec<Eval> = files
            .into_iter()
            .map(|file| {
                let mut possible_board = *self.board;
                possible_board.insert(file, self.color);
                let eval = minimax(
                    &mut possible_board,
                    self.color.other(),
                    self.depth,
                    i32::MIN,
                    i32::MAX,
                );
                Eval(file, eval)
            })
            .collect();

        match self.color {
            Color::Red => evaluations.sort_unstable_by(|a, b| b.cmp(a)),
            Color::Yellow => evaluations.sort_unstable(),
        }

        evaluations[0].0
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Eval(usize, i32);

impl Ord for Eval {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Eval {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn minimax(board: &mut Board, color: Color, depth: usize, alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        if board.has_connect_4(color.other()) {
            return match color {
                Color::Red => -100,
                Color::Yellow => 100,
            };
        } else {
            return 0;
        }
    }

    let mut alpha = alpha;
    let mut beta = beta;

    match color {
        Color::Red => {
            let mut highest_score = i32::MIN;
            for file in board.legal_files() {
                board.insert(file, color);
                let score = minimax(board, color.other(), depth - 1, alpha, beta);
                board.remove(file);

                highest_score = cmp::max(score, highest_score);
                alpha = cmp::max(highest_score, alpha);

                if beta <= alpha {
                    break;
                }
            }

            highest_score
        }
        Color::Yellow => {
            let mut lowest_score = i32::MAX;
            for file in board.legal_files() {
                board.insert(file, color);
                let score = minimax(board, color.other(), depth - 1, alpha, beta);
                board.remove(file);

                lowest_score = cmp::min(score, lowest_score);
                beta = cmp::min(lowest_score, beta);

                if beta <= alpha {
                    break;
                }
            }

            lowest_score
        }
    }
}
