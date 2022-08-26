use std::io::{self, Write};

mod models;
use models::*;

fn main() {
    game_loop();
}

fn game_loop() {
    let mut board = Board::new();
    let mut current_player = Cell::X;
    let mut running = true;
    let mut game_lap = 0;

    while running {
        game_lap += 1;

        let responce = ask_player(&current_player, game_lap, &board);
        board.play_move(responce, &current_player);
        current_player = increment_player(current_player);
        println!("{}", board.print());
    }
}

fn ask_player(player: &Cell, game_lap: u8, board: &Board) -> Place {
    loop {
        let row = input_point(&player, game_lap, "row");
        let collum = input_point(&player, game_lap, "collum");
        let place = Place {row, collum};
        if is_valid(&place, &board) {break place;}
    }
}

fn input_point(player: &Cell, game_lap: u8, place_holder: &str) -> Point{
    loop {
        let mut buffer = String::new();
        print!("[{}] Player {} your turn, {} number (1, 2, 3): ",
                 game_lap,  player.print(),place_holder);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from stdin");
        
        if let Ok(i) = buffer.trim().parse::<u8>() {
            match i {
                1 => break Point::I,
                2 => break Point::Ii,
                3 => break Point::Iii,
                _ => {
                    println!("(1, 2, 3)!");
                    continue;
                }
            }
        } else {
            println!("Invalid Input");
            continue;
        }
    }
}

fn is_valid(place: &Place, board: &Board) -> bool {
    board[place] == Cell::N
}

fn increment_player(current_player: Cell) -> Cell {
    if current_player == Cell::X {
        Cell::O
    } else {
        Cell::X
    }
}