use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use crate::gamestate::GameState;

pub type Table = RwLock<HashMap<GameState, Entry>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Flag {
    Exact,
    UpperBound,
    LowerBound,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    gamestate: GameState,
    best_move: u8,
    eval: i32,
}

impl Entry {
    pub fn new(gamestate: GameState, best_move: u8, eval: i32) -> Self {
        Entry {
            gamestate, 
            best_move,
            eval
        }
    }    
}

pub fn new_table() -> Table {
    RwLock::new(HashMap::new())
}

pub fn store_entry(table: &Table, entry: Entry) {
    let mut table = table.write().expect("rw lock on tt to not be poisoned");
    table.insert(entry.gamestate, entry); // TODO fix error with a borrow or something?
}

pub fn probe_eval(table: &Table, gamestate: &GameState) -> Option<u8> {
    let table = table.read().expect("rw lock on tt to not be poisoned");
    let entry = table.get(gamestate);
    entry.map(|entry| entry.best_move)
}