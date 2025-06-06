use crate::color::Gameover;
use crate::gamestate::GameState;
use crate::transposition;
use rayon::prelude::*;

// Prioritize columns near the center
const COLUMN_ORDERING: [u8; 7] = [3, 2, 4, 1, 5, 0, 6];
const WINNING_EVAL: i32 = 1000; // Value for a winning move

pub fn negamax_entrypoint(board: &GameState, table: &mut transposition::Table) -> (u8, i32) {
    let mut results: Vec<(u8, i32)> = COLUMN_ORDERING
        .into_par_iter()
        // .into_iter()
        .filter(|&column| board.get_height(column) < 6) // filter out full columns
        .map(|column| {
            // Evaluate each possible move
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = -negamax(&mut new_board, -10_000, 10_000, 0, table);
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

fn negamax(
    board: &mut GameState,
    mut alpha: i32,
    beta: i32,
    ply: u16,
    table: &transposition::Table,
) -> i32 {
    // Probe the transposition table to see if we have encountered this game state before
    if let Some(eval) = transposition::probe_eval(table, board) {
        return eval;
    }

    // Base case: game is over
    let eval: Option<i32> = match board.gameover_state() {
        Gameover::Win(color) => {
            // If the game has ended, then the next person to play has lost
            debug_assert_ne!(
                color,
                board.current_player(),
                "Gameover state should not be Win for current player"
            );
            Some(-WINNING_EVAL + ply as i32)
        }
        Gameover::Tie => Some(0),
        Gameover::None => None,
    };

    // If this `game_state` is terminal, store the eval in the transposition table
    if let Some(eval) = eval {
        transposition::store_entry(table, board, eval, None);
        return eval;
    }

    let mut max_eval = -20_000;
    for column in COLUMN_ORDERING {
        if board.get_height(column) < 6 {
            board.make_move(column);
            let eval = -negamax(board, -beta, -alpha, ply + 1, table);
            board.undo_move(column);
            max_eval = max_eval.max(eval);

            alpha = alpha.max(max_eval);
            if alpha >= beta {
                break;
            }
        }
    }

    transposition::store_entry(table, board, max_eval, None);

    max_eval
}
