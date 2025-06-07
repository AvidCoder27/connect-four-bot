pub mod color;
mod engine;
mod gamestate;
mod transposition;

use core::panic;
use std::ops::ControlFlow;

use color::{Color, Gameover};
use gamestate::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gamemode {
    PlayerVsPlayer,
    PlayerVsComputer,
    ComputerVsComputer,
}

fn main() {
    run_game();
}

fn run_game() -> Option<()> {
    println!("\n==========CONNECT FOUR==========");
    println!("Enter 'q' at any time to quit the game.");

    let mut transposition_table = transposition::new_table();
    let mut board = load_game()?;
    let (mut gamemode, mut player_color) = determine_gamemode()?;
    override_starting_color(&mut board)?;

    println!("\nStarting game in {:?} mode", gamemode);
    println!("Initial board state:");
    println!("{:?}", board);
    println!();

    loop {
        match gamemode {
            Gamemode::PlayerVsPlayer => {
                let ctrl = make_player_turn(&mut board);
                if let ControlFlow::Break(_) = ctrl {
                    break;
                }
                if let ControlFlow::Continue(b) = ctrl {
                    if b {
                        // Switch to computer mode
                        player_color = board.current_player.opposite();
                        gamemode = Gamemode::PlayerVsComputer;
                        println!(
                            "Switching to Human vs Computer mode. Computer will play as {}.",
                            player_color.opposite()
                        );
                    }
                }
            }
            Gamemode::PlayerVsComputer => {
                if board.current_player == player_color {
                    if let ControlFlow::Break(_) = make_player_turn(&mut board) {
                        break;
                    }
                } else {
                    make_computer_turn(&mut board, &mut transposition_table);
                }
            }
            Gamemode::ComputerVsComputer => {
                make_computer_turn(&mut board, &mut transposition_table);
            }
        }

        match board.gameover_state() {
            Gameover::Win(color) => {
                println!("\nGame Over! {} wins!", color);
                break;
            }
            Gameover::Tie => {
                println!("\nGame Over! It's a tie!");
                break;
            }
            Gameover::None => {}
        }
    }

    println!("Final board state:");
    println!("{}", board.to_fen());
    return None;
}

fn load_game() -> Option<GameState> {
    println!("Would you like to load a game from FEN? (y/n)");
    let input = read_input()?;

    if input == "y" {
        println!("Enter FEN string:");
        let input = read_input()?;
        let board = GameState::from_fen(input.as_str(), None);
        println!("Loaded game state from FEN:");
        println!("{:?}", board);
        Some(board)
    } else if input == "q" {
        return None;
    } else {
        println!("Starting a new game.");
        Some(GameState::new())
    }
}

fn override_starting_color(board: &mut GameState) -> Option<()> {
    println!(
        "Would you like {} or {} to play first? (y/r)",
        Color::Yellow,
        Color::Red
    );
    let input = read_input()?;
    if input == "y" {
        board.override_current_player(Color::Yellow);
    } else {
        board.override_current_player(Color::Red);
    }
    println!(
        "Starting with {} as the first player.",
        board.current_player
    );
    Some(())
}

fn read_input() -> Option<String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();
    if input == "q" {
        println!("Exiting game.");
        return None;
    }
    Some(input)
}

fn make_computer_turn(board: &mut GameState, transposition_table: &mut transposition::Table) {
    println!("\n{} Computer turn", board.current_player);
    let (column, eval) = engine::negamax_entrypoint(board, transposition_table);
    if board.make_move(column as u8) {
        println!(
            "{} plays column {} with eval of {}",
            board.current_player.opposite(),
            column + 1, // Convert to 1-indexed for display
            eval
        );
        println!("\n{:?}", board);
    } else {
        panic!("Computer tried to play in a full column: {}", column + 1);
    }
}

/// If all goes well, returns a ControlFlow::Continue(false) to switch to playing against the bot.
/// If the user wants to quit, returns ControlFlow::Break(()).
/// If the user wants to switch to playing against the bot, returns ControlFlow::Continue(true).
fn make_player_turn(board: &mut GameState) -> ControlFlow<(), bool> {
    println!("\n{}'s turn", board.current_player);
    println!("Enter column number (1-7) or 'q' to quit or 's' to switch to playing against bot:");
    let input = match read_input() {
        Some(input) => input,
        None => return ControlFlow::Break(()), // User wants to quit
    };
    if input == "s" {
        return ControlFlow::Continue(true);
    }

    // User inputs 1-indexed column
    match input.parse::<u8>() {
        Ok(column) if column < 8 && column > 0 => {
            if board.make_move(column - 1) {
                println!("\n{:?}", board);
            } else {
                println!("Column {} is full!", column);
            }
        }
        _ => println!("Please enter a valid column number (1-7)"),
    }
    ControlFlow::Continue(false)
}

fn determine_gamemode() -> Option<(Gamemode, Color)> {
    println!("Select game mode:");
    println!("1. Human vs Human");
    println!("2. Human vs Computer");
    println!("3. Computer vs Computer");

    let gamemode = loop {
        let input = read_input()?;

        if input.is_empty() {
            println!("Defaulting to Human vs Computer.");
            break Gamemode::PlayerVsComputer;
        }

        match input.parse() {
            Ok(1) => break Gamemode::PlayerVsPlayer,
            Ok(2) => break Gamemode::PlayerVsComputer,
            Ok(3) => break Gamemode::ComputerVsComputer,
            _ => {
                println!("Invalid option, select 1, 2, or 3");
                continue;
            }
        };
    };

    let player_color = if gamemode == Gamemode::PlayerVsComputer {
        // get user input for player color
        println!(
            "Do you want to play as {} or {} (y/r)?",
            Color::Yellow,
            Color::Red
        );
        let input = read_input()?;
        match input.as_str() {
            "y" | "yellow" => Color::Yellow,
            "r" | "red" => Color::Red,
            _ => {
                println!("Defaulting to {}", Color::Red);
                Color::Red
            }
        }
    } else {
        Color::Yellow // Default color for Player vs Player and Computer vs Computer, doesn't matter
    };

    Some((gamemode, player_color))
}
