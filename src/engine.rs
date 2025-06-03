use crate::color::Gameover;
use crate::gamestate::GameState;
use rayon::prelude::*;

pub fn negamax_entrypoint(board: &GameState, depth: u8) -> u8 {
    let (best_move, _) = (0..7)
        .into_par_iter()
        .filter(|&column| board.get_height(column) < 6)
        .map(|column| {
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = negamax(&new_board, depth, i32::MIN + 1, i32::MAX);
            (column, eval)
        })
        .reduce(|| (8, i32::MIN), |a, b| if b.1 > a.1 { b } else { a });
    best_move
}

fn negamax(board: &GameState, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    // Base case: game is over
    match board.gameover_state() {
        Gameover::Win(color) => {
            // If the game has ended, then the next person to play has lost
            return if color == board.current_player() {
                -99999 // Opponent wins
            } else {
                99999 // Current player wins
            };
        }
        Gameover::Tie => return 0,
        Gameover::None => {}
    }

    // Tertiary Base case: if depth is 0, stop searching
    if depth == 0 {
        return evaluate(board);
    }

    let mut max_eval = i32::MIN + 1;

    for column in 0..7 {
        if board.get_height(column) < 6 {
            let mut new_board = board.clone();
            new_board.make_move(column);
            let eval = -negamax(&new_board, depth - 1, -beta, -alpha);

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

fn evaluate(_board: &GameState) -> i32 {
    // For now, a simple evaluation function that returns 0
    // This should be replaced with a more sophisticated evaluation function
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_wins() {
        let mut game = GameState::from_str("..ryyry/..rrryr/rryyyrr/yyyrryy/rryyyry/yyrrryr", None);
        println!("{:?}", game);

        //yellow to play, should block
        let m = negamax_entrypoint(&game, 5);
        game.make_move(m);
        println!("{:?}", game);
        
        // red to play, wins if yellow is stupid
        let m = negamax_entrypoint(&game, 5);
        game.make_move(m);
        println!("{:?}", game);
        
        assert_eq!(m, 1);
    }
}