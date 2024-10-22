use std::io::{stdout, stdin, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    K,
    Q,
    R,
    B,
    N,
    P,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub piece: Option<(Piece, bool)>,
}

#[derive(Debug, Clone, Copy)]
pub struct LastMove {
    pub position: (usize, usize),
    pub piece: Piece,
    pub color: bool,
}

#[derive(Debug)]
pub struct Board {
    pub squares: [[Square; 8]; 8],
    pub last_move: Option<LastMove>,
}

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
