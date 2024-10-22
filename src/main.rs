mod board;
mod bot;

use board::board::{Board, display_board, get_user_input};
use board::placement::parse_and_make_move;
use bot::easy::choose_bot_move;

fn main() {
    let mut board = Board::new();

    loop {
        display_board(&board);

        let user_move = get_user_input();
        if parse_and_make_move(&mut board, &user_move, true) {
            display_board(&board);
            let bot_move = choose_bot_move(&board);
            parse_and_make_move(&mut board, &bot_move, false);
        } else {
            println!("Invalid move! Please try again.");
        }
    }
}
