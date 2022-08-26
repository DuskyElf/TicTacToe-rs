use std::io::{self, Write};

mod models;
use models::*;

fn main() {
    let game_result = game_loop();
    
    match game_result.winner {
        Winner::Won(winner) => {
            println!("Player {} won the game in {} Game laps.", 
                             winner.print(),    game_result.game_lap);
            println!("Whooo!!");
        },
        Winner::Draw => {
            println!("Bruh, it was a draw after {} moves.", game_result.game_lap);
        },
    }
}

fn game_loop() -> GameResult {
    let mut board = Board::new();
    let mut current_player = Cell::X;
    let mut game_lap = 0;

    loop {
        game_lap += 1;

        let responce = ask_player(&current_player, game_lap, &board);
        board.play_move(responce, &current_player);
        current_player = increment_player(current_player);
        println!("{}", board.print());

        let winner = check_winner(&board);

        if winner != Cell::N {
            return GameResult {
                winner: Winner::Won(winner),
                game_lap,
            };
        }

        if check_draw(&board) {
            return GameResult {
                winner: Winner::Draw,
                game_lap,
            }
        }
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

fn check_winner(board: &Board) -> Cell {
    let b = board.board_state;

    for i in 0..3 {
        // Horizontal Checks
        if b[i][0] != Cell::N {
            if b[i][0] == b[i][1] && b[i][1] == b[i][2] {
                return b[i][0];
            }
        }

        // Vertical Checks
        if b[0][i] != Cell::N {
            if b[0][i] == b[1][i] && b[1][i] == b[2][i] {
                return b[0][i];
            }
        }
    }

    // Diagonal Checks
    if b[0][0] != Cell::N {
        if b[0][0] == b[1][1] && b[1][1] == b[2][2] {
            return b[0][0];
        }
    }

    if b[2][0] != Cell::N {
        if b[2][0] == b[1][1] && b[1][1] == b[0][2] {
            return b[2][0];
        }
    }

    Cell::N
}

fn check_draw(board: &Board) -> bool {
    for i in board.board_state {
        for cell in i {
            if cell == Cell::N {
                return false;
            }
        }
    }

    true
}
