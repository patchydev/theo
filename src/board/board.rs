use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {
    pub on_start: bool,
    pub can_ep: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    K,
    Q,
    R,
    B,
    N,
    P(Pawn),
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub piece: Option<(Piece, bool)>,
}

#[derive(Debug)]
pub struct Board {
    pub squares: [[Square; 8]; 8],
}

pub fn parse_position(pos: &str) -> (usize, usize) {
    let chars: Vec<char> = pos.chars().collect();
    if chars.len() != 2 {
        panic!("Invalid position: {}", pos); // You could handle this more gracefully
    }

    // Convert the column character ('a' - 'h') to a 0-based index
    let col = (chars[0] as u8 - b'a') as usize;
    // Convert the row character ('1' - '8') to a 0-based index (from the bottom of the board)
    let row = (8 - (chars[1] as u8 - b'0')) as usize;

    (row, col)
}

pub fn display_board(board: &Board) {
    for row in board.squares.iter() {
        for square in row.iter() {
            match square.piece {
                Some((Piece::P(_), true)) => print!("PW "),
                Some((Piece::P(_), false)) => print!("PB "),
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
    println!("Enter your move:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
