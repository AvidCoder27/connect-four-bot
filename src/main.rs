pub mod color;
mod engine;
mod gamestate;

use std::ops::ControlFlow;

use color::{Color, Gameover};
use gamestate::GameState;

const MAX_DEPTH: u8 = 20; // Maximum depth for the negamax algorithm

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gamemode {
    PlayerVsPlayer,
    PlayerVsComputer,
    ComputerVsComputer,
}

fn main() {
    println!("\n==========CONNECT FOUR==========");
    // let mut board = GameState::from_fen("...r.../...y.../...r.../..yy.y./..yr.r./.yryrr.", Some(Color::Red));
    let mut board = GameState::new();

    load_game(&mut board);

    println!("Select game mode:");
    println!("1. Human vs Human");
    println!("2. Human vs Computer");
    println!("3. Computer vs Computer");

    let (mut gamemode, mut player_color) = determine_gamemode();

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
                        player_color = board.current_player().opposite();
                        gamemode = Gamemode::PlayerVsComputer;
                        println!("Switching to Human vs Computer mode. Computer will play as {}.", player_color.opposite());
                    }
                }
            }
            Gamemode::PlayerVsComputer => {
                if board.current_player() == player_color {
                    if let ControlFlow::Break(_) = make_player_turn(&mut board) {
                        break;
                    }
                } else {
                    make_computer_turn(&mut board);
                }
            }
            Gamemode::ComputerVsComputer => {
                make_computer_turn(&mut board);
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
}

fn load_game(board: &mut GameState) {
    println!("Would you like to load a game from FEN? (y/n)");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();
    if input == "y" {
        println!("Enter FEN string:");
        let mut fen_input = String::new();
        std::io::stdin()
            .read_line(&mut fen_input)
            .expect("Failed to read input");
        *board = GameState::from_fen(fen_input.trim(), None);
        println!("Loaded game state from FEN:");
        println!("{:?}", board);
    } else {
        println!("Starting a new game.");
    }
}

fn make_computer_turn(board: &mut GameState) {
    println!("\nComputer's turn");
    let (column, eval) = engine::negamax_entrypoint(&*board, MAX_DEPTH);
    if board.make_move(column as u8) {
        println!("{} plays column {} with eval of {}", board.current_player().opposite(), column, eval);
        println!("\n{:?}", board);
    } else {
        println!("Column {} is full!", column);
    }
}

fn make_player_turn(board: &mut GameState) -> ControlFlow<(), bool> {
    let mut input = String::new();
    println!("\n{}'s turn", board.current_player());
    println!("Enter column number (0-6) or 'q' to quit or 's' to switch to playing a bot:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();
    if input == "q" {
        return ControlFlow::Break(());
    }
    if input == "s" {
        return ControlFlow::Continue(true);
    }

    match input.parse::<u8>() {
        Ok(column) if column < 7 => {
            if board.make_move(column) {
                println!("\n{:?}", board);
            } else {
                println!("Column {} is full!", column);
            }
        }
        _ => println!("Please enter a valid column number (0-6)"),
    }
    ControlFlow::Continue(false)
}

fn determine_gamemode() -> (Gamemode, Color) {
    let gamemode = loop {
        let mut gamemode = String::new();
        std::io::stdin()
            .read_line(&mut gamemode)
            .expect("Failed to read input");

        match gamemode.trim().parse() {
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
        println!("Do you want to play as Yellow (Y) or Red (R)?");
        let mut color_input = String::new();
        std::io::stdin()
            .read_line(&mut color_input)
            .expect("Failed to read input");
        match color_input.trim().to_lowercase().as_str() {
            "y" | "yellow" => Color::Yellow,
            "r" | "red" => Color::Red,
            _ => {
                println!("Invalid input, defaulting to Yellow");
                Color::Yellow
            }
        }
    } else {
        Color::Yellow // Default color for Player vs Player and Computer vs Computer, doesn't matter
    };

    (gamemode, player_color)
}
