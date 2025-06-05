use crate::color::Gameover;
use crate::gamestate::GameState;
use rayon::prelude::*;

// Prioritize columns near the center
const COLUMN_ORDERING: [u8; 7] = [3, 2, 4, 1, 5, 0, 6];
const WINNING_EVAL: i32 = 1000; // Value for a winning move
const MAX_PLY: u16 = 50;

pub fn negamax_entrypoint(board: &GameState) -> (u8, i32) {
    let mut results: Vec<(u8, i32)> = COLUMN_ORDERING
        .into_par_iter()
        // .into_iter()
        .filter(|&column| board.get_height(column) < 6) // filter out full columns
        .map(|column| {
            // Evaluate each possible move
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = -negamax(&new_board, -10_000, 10_000, 0);
            println!("Column {} evaluated to {}", column + 1, eval);
            (column, eval)
        })
        .collect();

    results.sort_by_key(|result| -result.1);

    println!();
    for (col, eval) in results.iter() {
        println!("Column {} evaluated to {}", col + 1, eval);
    }

    *results.first().expect("Must have at least one valid move")
}

fn negamax(board: &GameState, mut alpha: i32, beta: i32, ply: u16) -> i32 {
    // Base case: game is over
    match board.gameover_state() {
        Gameover::Win(color) => {
            // If the game has ended, then the next person to play has lost
            debug_assert_ne!(
                color,
                board.current_player(),
                "Gameover state should not be Win for current player"
            );
            return -WINNING_EVAL + ply as i32;
        }
        Gameover::Tie => return 0,
        Gameover::None => {}
    }

    if ply >= MAX_PLY {
        return 0;
    }

    let mut max_eval = -20_000;
    for column in COLUMN_ORDERING {
        if board.get_height(column) < 6 {
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = -negamax(&new_board, -beta, -alpha, ply + 1);
            max_eval = max_eval.max(eval);

            alpha = alpha.max(max_eval);
            if alpha >= beta {
                break;
            }
        }
    }

    max_eval
}
