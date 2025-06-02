pub mod color;
mod engine;
mod gamestate;
use gamestate::GameState;

fn main() {
    println!("\n==========CONNECT FOUR==========");
    let mut board = GameState::new();
    println!("\n{:?}", board);

    loop {
        let mut input = String::new();
        println!("\n{}'s turn", board.current_player());
        println!("Enter column number (0-6) or 'q' to quit:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input == "q" {
            break;
        }

        match input.parse::<u8>() {
            Ok(column) if column < 7 => {
                if board.make_move(column) {
                    println!("\n{:?}", board);
                } else {
                    println!("Column {} is full!", column);
                }
            },
            _ => println!("Please enter a valid column number (0-6)"),
        }
    }
}
