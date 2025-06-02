pub mod color;
mod engine;
mod gamestate;

use std::ops::ControlFlow;

use color::{Color, Gameover};
use gamestate::GameState;

const MAX_DEPTH: u8 = 5; // Maximum depth for the negamax algorithm

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gamemode {
    PlayerVsPlayer,
    PlayerVsComputer,
    ComputerVsComputer,
}

fn main() {
    println!("\n==========CONNECT FOUR==========");
    let mut board = GameState::new();

    println!("Select game mode:");
    println!("1. Human vs Human");
    println!("2. Human vs Computer");
    println!("3. Computer vs Computer");

    let (gamemode, player_color) = determine_gamemode();

    println!("\nStarting game in {:?} mode", gamemode);
    println!("{:?}", board);

    loop {
        match gamemode {
            Gamemode::PlayerVsPlayer => {
                if let ControlFlow::Break(_) = make_player_turn(&mut board) {
                    break;
                }
            }
            Gamemode::PlayerVsComputer => {
                if board.current_player() == player_color {
                    if let ControlFlow::Break(_) = make_player_turn(&mut board) {
                        break;
                    }
                } else {
                    // Computer's turn
                    println!("\nComputer's turn");
                    let column = engine::negamax_entrypoint(&board, MAX_DEPTH);
                    if board.make_move(column as u8) {
                        println!("\n{:?}", board);
                    } else {
                        println!("Column {} is full!", column);
                    }
                }
            }
            Gamemode::ComputerVsComputer => {
                // Computer vs Computer logic
                println!("{:?}", board);
                let column = engine::negamax_entrypoint(&board, MAX_DEPTH);
                if board.make_move(column as u8) {
                    println!("\n{:?}", board);
                } else {
                    panic!("Computer tried to play in a full column!");
                }
            }
        }

        if board.gameover_state() != Gameover::None {
            break;
        }
    }
}

fn make_player_turn(board: &mut GameState) -> ControlFlow<()> {
    let mut input = String::new();
    println!("\n{}'s turn", board.current_player());
    println!("Enter column number (0-6) or 'q' to quit:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();
    if input == "q" {
        return ControlFlow::Break(());
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
    ControlFlow::Continue(())
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
