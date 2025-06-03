use crate::color::{Color, Gameover};
use colored::*;
use std::fmt;

const FULL_BOARD_MASK: u64 = 0b_0111111_0111111_0111111_0111111_0111111_0111111_0111111; // 7 bits per column, MSB is sentinel

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

    pub fn override_current_player(&mut self, color: Color) {
        self.current_player = color;
    }

    pub fn from_fen(s: &str, color: Option<Color>) -> Self {
        let mut game = GameState::new();
        let mut row = 5; // Start from top row
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

        // Count pieces to determine current player
        let red_count = game.red.count_ones();
        let yellow_count = game.yellow.count_ones();
        if let Some(c) = color {
            game.current_player = c;
        } else {
            game.current_player = if red_count == yellow_count {
                Color::Yellow
            } else {
                Color::Red
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

        if piece_count == 42{ // 6 rows * 7 columns = 42 total pieces {
            Gameover::Tie
        } else {
            Gameover::None
        }
    }

    /// Bitboard win detection for a single player's board.
    fn has_won(board: u64) -> bool {
        // Directions: right (1), down (7), down-right (6), down-left (8)
        const DIRECTIONS: [u32; 4] = [1, 7, 6, 8];

        for &dir in &DIRECTIONS {
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

        writeln!(f, "  1  2  3  4  5  6  7") // Column indices
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        gamestate::{GameState, Gameover},
    };

    #[test]
    fn test_game_not_over_empty_board() {
        let game = GameState::new();
        assert_eq!(game.gameover_state(), Gameover::None);
    }

    #[test]
    fn test_red_wins_vertically() {
        let mut game = GameState::new();
        game.red = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3); // vertical win
        assert_eq!(game.gameover_state(), Gameover::Win(Color::Red));
    }

    #[test]
    fn test_yellow_wins_horizontally() {
        let mut game = GameState::new();
        game.yellow = (1 << 0) | (1 << 7) | (1 << 14) | (1 << 21); // horizontal win
        assert_eq!(game.gameover_state(), Gameover::Win(Color::Yellow));
    }

    #[test]
    fn test_red_wins_diagonal_up_right() {
        let mut game = GameState::new();
        game.red = (1 << 0) | (1 << 8) | (1 << 16) | (1 << 24); // up-right diagonal
        assert_eq!(game.gameover_state(), Gameover::Win(Color::Red));
    }

    #[test]
    fn test_yellow_wins_diagonal_up_left() {
        let mut game = GameState::new();
        game.yellow = (1 << 21) | (1 << 15) | (1 << 9) | (1 << 3); // up-left diagonal
        assert_eq!(game.gameover_state(), Gameover::Win(Color::Yellow));
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
