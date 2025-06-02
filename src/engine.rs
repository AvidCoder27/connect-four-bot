pub fn negamax(
    board: &mut crate::gamestate::GameState,
    depth: u8,
    mut alpha: i32,
    beta: i32,
) -> i32 {
    if depth == 0 || board.filled() == 0x3FFFFFFF {
        return evaluate(board);
    }

    let mut max_eval = i32::MIN;

    for column in 0..7 {
        if board.get_height(column) < 6 {
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = -negamax(&mut new_board, depth - 1, -beta, -alpha);

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

fn evaluate(board: &crate::gamestate::GameState) -> i32 {
    0
}