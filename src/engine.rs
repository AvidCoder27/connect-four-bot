use crate::color::Gameover;
use crate::gamestate::GameState;
use crate::transposition;
use rayon::prelude::*;
use tinyvec::ArrayVec;

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
            // We must check for terminal states because negamax does not check itself for termination.
            if let Some(eval) = evaluate_termination(&mut new_board, 0, table) {
                (column, eval)
            } else {
                let eval = -negamax(&mut new_board, -10_000, 10_000, 0, table);
                (column, eval)
            }
        })
        .collect();

    results.sort_by_key(|result| -result.1);

    println!();
    for (col, eval) in results.iter() {
        println!("Column {} evaluated to {}", col + 1, eval);
    }

    *results.first().expect("Must have at least one valid move")
}

/// Eagerly evaluate the board for a winning move.
/// This function checks if the `board` is game over
/// and returns the evaluation if it is, otherwise returns None.
/// Additionally, it stores the evaluation in the transposition table if the `game_state` is terminal.
fn evaluate_termination(
    board: &mut GameState,
    ply: u16,
    table: &transposition::Table,
) -> Option<i32> {
    let eval = match board.gameover_state() {
        Gameover::Win(color) => {
            // If the game has ended, then the next person to play has lost
            debug_assert_ne!(
                color,
                board.current_player(),
                "Gameover state should not be Win for current player"
            );
            // The current player has lost, so we return a negative eval
            // We add on the ply to encourage dragging out losing games
            Some(ply as i32 - WINNING_EVAL)
        }
        Gameover::Tie => Some(0),
        Gameover::None => None,
    }?;
    transposition::store_entry(table, board, eval, None);
    Some(eval)
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

    // We don't need to check for a game over state here, because we already did from the parent node.
    // // Base case: the game is over
    // if let Some(eval) = evaluate_termination(board, ply, table) {
    //     return eval;
    // }

    // Generate all legal moves
    let legal_moves: ArrayVec<[u8; 7]> = COLUMN_ORDERING
        .iter()
        .filter(|&&column| board.get_height(column) < 6)
        .copied()
        .collect();

    // First, eagerly evaluate the board for a winning move
    // This is basically just calling the first half of `negamax` on each legal move
    // If we find a winning move, we can return immediately
    for &column in legal_moves.iter() {
        board.make_move(column);
        let eval = evaluate_termination(board, ply + 1, table);
        board.undo_move(column);
        if let Some(eval) = eval {
            // We found a winning move, so we can return it immediately.
            // This could also be a tie, in which case we still return it.
            // This is because when a move causes a tie, it's because it's the last move (and doesn't cause a win)
            // A tie move is also necessarily the only possible move, so we can return it immediately.
            return eval;
        }
    }

    // Continue down the negamax tree, evaluating each move recursively
    let mut max_eval = -20_000;
    for column in legal_moves {
        board.make_move(column);
        let eval = -negamax(board, -beta, -alpha, ply + 1, table);
        board.undo_move(column);
        max_eval = max_eval.max(eval);

        alpha = alpha.max(max_eval);
        if alpha >= beta {
            break;
        }
    }

    transposition::store_entry(table, board, max_eval, None);

    max_eval
}
