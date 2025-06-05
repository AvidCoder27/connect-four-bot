use crate::gamestate::GameState;

struct TranspositionEntry {
    pub gamestate: GameState,
    pub best_move: u8,
    // The best move for the current player in this gamestate
    pub eval: i32,
}
