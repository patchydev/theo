use colored::Colorize;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::Board;
use crate::board::move_generation::generate_valid_moves;

pub fn choose_bot_move(board: &mut Board) -> String {
    let valid_moves = generate_valid_moves(board, false);
    if valid_moves.is_empty() {
        //return String::from("No valid moves available.");
        return format!("{}", "No valid moves available.".blue().bold())
    }

    let mut rng = thread_rng();
    let random_move = valid_moves.choose(&mut rng).unwrap();

    let from_notation = format!("{}{}", (random_move.0.1 as u8 + 'a' as u8) as char, 8 - random_move.0.0);
    let to_notation = format!("{}{}", (random_move.1.1 as u8 + 'a' as u8) as char, 8 - random_move.1.0);

    format!("{} {}", from_notation, to_notation)
}