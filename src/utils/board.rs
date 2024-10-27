use std::io::{stdout, stdin, Write};

use crate::{board::board::Piece, Board};

pub fn parse_position(pos: &str) -> (usize, usize) {
    let chars: Vec<char> = pos.chars().collect();
    if chars.len() != 2 {
        panic!("Invalid position: {}", pos);
    }

    let col = (chars[0] as u8 - b'a') as usize;
    let row = (8 - (chars[1] as u8 - b'0')) as usize;

    (row, col)
}

pub fn display_board(board: &Board) {
    for row in board.squares.iter() {
        for square in row.iter() {
            match square.piece {
                Some((piece, true)) => print!("{:?}W ", piece),
                Some((piece, false)) => print!("{:?}B ", piece),
                None => print!("-- "),
            }
        }
        println!();
    }
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    print!("Enter your move: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn choose_promotion_piece() -> Result<Piece, String> {
    let mut input = String::new();
    println!("You're about to promote a pawn!\n
        1. R\n
        2. B\n
        3. N\n
        4. Q");

    print!("Please enter your choice: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // TODO: add error handling
    input = input.trim().to_string();

    match input.to_string().as_str() {
        "1" => Ok(Piece::R),
        "2" => Ok(Piece::B),
        "3" => Ok(Piece::N),
        "4" => Ok(Piece::Q),
        _ => Err("Invalid input to choose promotion piece".to_string()),
    }
}
