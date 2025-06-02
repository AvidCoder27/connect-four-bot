use crate::color::Color;
use colored::*;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct GameState {
    // Bitboards for each player, using 7 bits per column (6 rows + 1 sentinel row for overflow)
    red: u64,
    yellow: u64,
    current_player: Color,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            red: 0,
            yellow: 0,
            current_player: Color::Yellow,
        }
    }

    /// Make a move in the specified column.
    ///
    /// Returns `true` if the move was successful, `false` if the column is full.
    pub fn make_move(&mut self, column: u8) -> bool {
        let height = self.get_height(column);
        if height >= 6 {
            return false; // Column is full
        }

        let bit_index = column * 7 + height;
        let mask = 1u64 << bit_index;

        // place the piece in the appropriate player's bitboard
        match self.current_player {
            Color::Yellow => {
                self.yellow |= mask;
                self.current_player = Color::Red;
            }
            Color::Red => {
                self.red |= mask;
                self.current_player = Color::Yellow;
            }
        }
        true // Move was successful
    }

    pub fn get_height(&self, column: u8) -> u8 {
        let col_bits = (self.filled() >> (column * 7)) & 0x3F;
        if col_bits == 0x3F {
            return 6; // Column is full
        }
        col_bits.trailing_ones() as u8
    }

    pub fn filled(&self) -> u64 {
        self.red | self.yellow
    }

    pub fn current_player(&self) -> Color {
        self.current_player
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?} to play", self.current_player)?;

        for row in (0..6).rev() {
            // Print from top to bottom
            write!(f, "|")?;
            for col in 0..7 {
                let bit_index = col * 7 + row;
                let mask = 1u64 << bit_index;
                write!(
                    f,
                    " {} ",
                    if self.red & mask != 0 {
                        "R".red()
                    } else if self.yellow & mask != 0 {
                        "Y".yellow()
                    } else {
                        " ".white()
                    }
                )?;
            }
            writeln!(f, "|")?;
        }

        writeln!(f, "  0  1  2  3  4  5  6") // Column indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn game_with_column(column: u8, pieces: u8) -> GameState {
        let mut game = GameState::new();
        for _ in 0..pieces {
            game.make_move(column);
        }
        game
    }

    #[test]
    fn test_empty_column() {
        let game = GameState::new();
        assert_eq!(game.get_height(0), 0);
        assert_eq!(game.get_height(3), 0);
    }

    #[test]
    fn test_partially_filled_column() {
        let game = game_with_column(2, 3); // Place 3 pieces in column 2
        assert_eq!(game.get_height(2), 3);
    }

    #[test]
    fn test_filled_column() {
        let game = game_with_column(4, 6); // Fill column 4 completely
        assert_eq!(game.get_height(4), 6);
    }

    #[test]
    fn test_overfilled_column_still_returns_6() {
        let game = game_with_column(5, 7); // Add 7 moves to column 5 — should not happen
        assert_eq!(game.get_height(5), 6); // Still return 6, don't panic
    }

    #[test]
    fn test_mixed_columns() {
        let mut game = GameState::new();
        game.make_move(0); // Y
        game.make_move(1); // R
        game.make_move(0); // Y
        game.make_move(1); // R
        game.make_move(0); // Y
        println!("{:?}", game);
        assert_eq!(game.get_height(0), 3);
        assert_eq!(game.get_height(1), 2);
        assert_eq!(game.get_height(2), 0);
    }
}
