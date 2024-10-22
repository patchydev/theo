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
        .expect("Failed to read line");
    input = input.trim().to_string();

    if input == "1" {
        return Ok(Piece::R)
    } else if input == "2" {
        return Ok(Piece::B)
    } else if input == "3" {
        return Ok(Piece::N)
    } else if input == "4" {
        return Ok(Piece::Q)
    } else {
        return Err("Invalid input to choose promotion piece".to_string())
    }

}
