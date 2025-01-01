use std::time::Instant;

use connect_4::{
    board::{Board, Color},
    repl::Repl,
};

fn main() {
    // perft_test(10);
    let repl = Repl::new();
    repl.start();
}

fn perft_test(depth: usize) {
    println!("Beginning perft test to depth {}", depth);
    let start = Instant::now();

    let mut board = Board::new();

    let count = perft(depth, &mut board, Color::Yellow);

    let elapsed = start.elapsed();

    println!("Total moves generated: {}", count);
    println!("Time elapsed: {}ms", elapsed.as_millis());
    println!("Average NPS: {}", count as f32 / elapsed.as_secs_f32())
}

fn perft(depth: usize, board: &mut Board, color: Color) -> usize {
    let mut positions = 1;

    if depth == 0 || board.has_connect_4(color.other()) {
        return positions;
    }

    let moves = board.legal_files();
    for m in moves {
        board.insert(m, color);
        positions += perft(depth - 1, board, color.other());
        board.remove(m);
    }

    positions
}
