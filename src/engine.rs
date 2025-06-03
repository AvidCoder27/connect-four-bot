use crate::color::Gameover;
use crate::gamestate::GameState;
use rayon::prelude::*;

// Prioritize columns near the center
const COLUMN_ORDERING: [u8; 7] = [3, 2, 4, 1, 5, 0, 6];
const WINNING_EVAL: i32 = 100; // Value for a winning move
const MAX_DEPTH: u8 = 150; // effectively infinite depth for practical purposes

pub fn negamax_entrypoint(board: &GameState) -> (u8, i32) {
    let outcome = COLUMN_ORDERING
        .into_par_iter()
        // .into_iter()
        .filter(|&column| board.get_height(column) < 6) // filter out full columns
        .map(|column| {
            // Evaluate each possible move
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = negamax(&new_board, MAX_DEPTH, i32::MIN + 1, i32::MAX);
            println!(
                "Evaluating column {}: eval = {}",
                column + 1, eval, // Convert to 1-indexed for display
            );
            (column, eval)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("a valid move should always be found");
    outcome
}

fn negamax(board: &GameState, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    // Base case: game is over
    match board.gameover_state() {
        Gameover::Win(color) => {
            // If the game has ended, then the next person to play has lost
            return if color == board.current_player() {
                -WINNING_EVAL // Opponent wins
            } else {
                WINNING_EVAL // Current player wins
            };
        }
        Gameover::Tie => return 0,
        Gameover::None => {}
    }

    // Tertiary Base case: if depth is 0, give up
    if depth == 0 {
        return 0;
    }

    let mut max_eval = i32::MIN + 1;

    for column in COLUMN_ORDERING {
        if board.get_height(column) < 6 {
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = -negamax(&new_board, depth - 1, -beta, -alpha);

            if eval >= WINNING_EVAL || eval <= -WINNING_EVAL {
                return eval; // Found a game-ending move
            }

            max_eval = max_eval.max(eval);
            if max_eval >= beta {
                break; // Beta cut-off
            }
            if max_eval > alpha {
                alpha = max_eval; // Update alpha
            }
        }
    }

    max_eval
}
