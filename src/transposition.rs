use crate::color::Color;
use crate::gamestate::GameState;
use rand_mt::Mt64;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub type Table = RwLock<HashMap<u64, Entry>>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    hash: u64,
    eval: i32,
    best_move: Option<u8>,
}

pub fn new_table() -> Table {
    RwLock::new(HashMap::new())
}

pub fn store_entry(table: &Table, gamestate: &GameState, eval: i32, best_move: Option<u8>) {
    let hash = compute_hash(gamestate);
    let entry = Entry {
        hash,
        eval,
        best_move,
    };
    let mut table = table.write().expect("rw lock on tt to not be poisoned");
    table.insert(entry.hash, entry);
}

pub fn probe_eval(table: &Table, gamestate: &GameState) -> Option<i32> {
    let hash = compute_hash(gamestate);
    let table = table.read().expect("rw lock on tt to not be poisoned");
    let entry = table.get(&hash)?;
    Some(entry.eval)
}

/// tuple.0 is a vector of (red, yellow) hashes for each square,
/// tuple.1 is the hashes for current_player, (red, yellow).
static ZOBRIST_TABLE: LazyLock<(Vec<(u64, u64)>, (u64, u64))> = LazyLock::new(|| {
    const BOARD_SIZE: usize = 49;
    let mut table = vec![(0u64, 0u64); BOARD_SIZE];
    let mut rng = Mt64::new_unseeded();
    for square in 0..BOARD_SIZE {
        table[square] = (rng.next_u64(), rng.next_u64());
    }
    (table, (rng.next_u64(), rng.next_u64()))
});

fn compute_hash(game_state: &GameState) -> u64 {
    let mut hash = 0u64;
    hash = hash_bitboard(hash, game_state.yellow, Color::Yellow);
    hash = hash_bitboard(hash, game_state.red, Color::Red);
    hash ^= match game_state.current_player {
        Color::Red => ZOBRIST_TABLE.1 .0,
        Color::Yellow => ZOBRIST_TABLE.1 .1,
    };
    hash
}

/// Hash a bitboard with a color, starting with the given hash and returning a new hash.
fn hash_bitboard(mut hash: u64, mut bitboard: u64, color: Color) -> u64 {
    while bitboard > 0 {
        let square = ZOBRIST_TABLE.0[bitboard.trailing_zeros() as usize];
        hash ^= match color {
            Color::Red => square.0,
            Color::Yellow => square.1,
        };
        bitboard &= bitboard - 1;
    }
    hash
}
