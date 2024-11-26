mod board;
mod bot;
mod utils;

use colored::Colorize;

use board::board::Board;
use board::placement::parse_and_make_move;
use bot::easy::choose_bot_move;
use utils::board::{display_board, get_user_input};

fn main() {
    let mut board = Board::new();

    loop {
        display_board(&mut board);

        let user_move = get_user_input();
        if parse_and_make_move(&mut board, &user_move, true) {
            let mut bot_move = choose_bot_move(&mut board);
             while !parse_and_make_move(&mut board, &bot_move, false) {
                bot_move = choose_bot_move(&mut board);
             }
        } else {
            println!("{}", "Invalid move! Please try again.".red().bold());
        }
    }
}
