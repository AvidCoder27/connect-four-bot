use crate::color::{Color, Gameover};
use colored::*;
use std::fmt;

const USE_ICONS: bool = false;
const DEFAULT_STARTING_PLAYER: Color = Color::Red;

const PIECE_ICON: &'static str = "●";
const EMPTY_ICON: &'static str = "○";

const RED_PIECE: &'static str = if USE_ICONS { PIECE_ICON } else { "R" };
const YELLOW_PIECE: &'static str = if USE_ICONS { PIECE_ICON } else { "Y" };
const EMPTY_PIECE: &'static str = if USE_ICONS { EMPTY_ICON } else { " " };
const FULL_BOARD_MASK: u64 = 0b_0111111_0111111_0111111_0111111_0111111_0111111_0111111; // 7 bits per column, MSB is sentinel

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    // Bitboards for each player, using 7 bits per column (6 rows + 1 sentinel row for overflow)
    // MS_7bits is the far right column, LS_7bits is the far left column
    pub red: u64,
    pub yellow: u64,
    pub current_player: Color,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            red: 0,
            yellow: 0,
            current_player: Color::Yellow,
        }
    }

    #[inline(always)]
    pub fn override_current_player(&mut self, color: Color) {
        self.current_player = color;
    }

    pub fn from_fen(s: &str, color: Option<Color>) -> Self {
        let mut game = GameState::new();
        let mut row = 5; // Start from the top row
        let mut col = 0;

        for c in s.chars() {
            match c {
                'r' => {
                    let bit_index = col * 7 + row;
                    game.red |= 1u64 << bit_index;
                    col += 1;
                }
                'y' => {
                    let bit_index = col * 7 + row;
                    game.yellow |= 1u64 << bit_index;
                    col += 1;
                }
                '.' => col += 1,
                '/' => {
                    row -= 1;
                    col = 0;
                }
                _ => continue,
            }
        }

        // Count pieces to determine the current player
        let red_count = game.red.count_ones();
        let yellow_count = game.yellow.count_ones();
        if let Some(c) = color {
            game.current_player = c;
        } else {
            game.current_player = if red_count == yellow_count {
                DEFAULT_STARTING_PLAYER
            } else {
                DEFAULT_STARTING_PLAYER.opposite()
            };
        }

        game
    }

    pub fn to_fen(&self) -> String {
        let mut result = String::new();
        for row in (0..6).rev() {
            for col in 0..7 {
                let bit_index = col * 7 + row;
                let mask = 1u64 << bit_index;
                result.push(if self.red & mask != 0 {
                    'r'
                } else if self.yellow & mask != 0 {
                    'y'
                } else {
                    '.'
                });
            }
            if row > 0 {
                result.push('/');
            }
        }
        result
    }

    pub fn gameover_state(&self) -> Gameover {
        let piece_count = (self.filled() & FULL_BOARD_MASK).count_ones();
        // Only check for gameover if there are at least 7 pieces on the board
        // This is assuming normal gameplay where players alternate turns
        // With only 6 pieces, no player can win
        if piece_count < 7 {
            return Gameover::None;
        }

        // The only valid gameover state is if the non-current player has won
        match self.current_player {
            Color::Yellow => {
                if Self::has_won(self.red) {
                    return Gameover::Win(Color::Red);
                }
            }
            Color::Red => {
                if Self::has_won(self.yellow) {
                    return Gameover::Win(Color::Yellow);
                }
            }
        }

        // 6 rows * 7 columns = 42 total pieces
        if piece_count == 42 {
            Gameover::Tie
        } else {
            Gameover::None
        }
    }

    #[inline(always)]
    /// Bitboard win detection for a single player's board.
    fn has_won(board: u64) -> bool {
        // Directions: right (1), down (7), down-right (6), down-left (8)
        const DIRECTIONS: [u8; 4] = [1, 7, 6, 8];
        for dir in DIRECTIONS {
            let m1 = board & (board >> dir);
            let m2 = m1 & (m1 >> (dir * 2));
            if m2 != 0 {
                return true;
            }
        }

        false
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
        true // The move was successful
    }

    /// Undo a move made at a specific column
    pub fn undo_move(&mut self, column: u8) {
        let bit_index = column * 7 + self.get_height(column) - 1;
        let mask = 1u64 << bit_index;
        match self.current_player {
            Color::Yellow => {
                self.red &= !mask;
                self.current_player = Color::Red;
            }
            Color::Red => {
                self.yellow &= !mask;
                self.current_player = Color::Yellow;
            }
        }
    }

    #[inline(always)]
    pub fn get_height(&self, column: u8) -> u8 {
        const LS_SIX_BITS: u64 = 0b111111;
        let col_bits = (self.filled() >> (column * 7)) & LS_SIX_BITS;
        col_bits.trailing_ones() as u8
    }

    #[inline(always)]
    pub fn filled(&self) -> u64 {
        self.red | self.yellow
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:} to play", self.current_player)?;

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
                        RED_PIECE.red()
                    } else if self.yellow & mask != 0 {
                        YELLOW_PIECE.yellow()
                    } else {
                        EMPTY_PIECE.white()
                    }
                )?;
            }
            writeln!(f, "|")?;
        }

        writeln!(f, "  1  2  3  4  5  6  7")?; // Column indices
        writeln!(f, "\n{}", self.to_fen()) // Print FEN representation
    }
}

#[cfg(test)]
mod tests {
    use crate::gamestate::{GameState, Gameover};

    #[test]
    fn test_game_not_over_empty_board() {
        let game = GameState::new();
        assert_eq!(game.gameover_state(), Gameover::None);
    }

    #[test]
    fn test_tie_full_board_no_winner() {
        let game = GameState::from_fen("yrryyry/ryrrryr/rryyyrr/yyyrryy/rryyyry/yyrrryr", None);
        println!("{:?}", game);
        assert_eq!(game.gameover_state(), Gameover::Tie);
    }

    #[test]
    fn test_game_in_progress_partial_board() {
        let mut game = GameState::new();
        game.red = (1 << 0) | (1 << 7); // two red moves
        game.yellow = 1 << 1; // one yellow move
        assert_eq!(game.gameover_state(), Gameover::None);
    }

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
        let game = game_with_column(4, 6); // Fill column 4
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
