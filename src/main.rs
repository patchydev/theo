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
    let mut game_active = true;

    while game_active {
        display_board(&mut board);
        
        if board.is_checkmate(true) {
            println!("{}", "Checkmate! Black wins!".green().bold());
            break;
        } else if board.is_stalemate(true) {
            println!("{}", "Stalemate! The game is a draw.".yellow().bold());
            break;
        }
        
        if board.w_in_check {
            println!("{}", "White is in check!".red().bold());
        }

        let user_move = get_user_input();
        if parse_and_make_move(&mut board, &user_move, true) {
            if board.is_checkmate(false) {
                display_board(&mut board);
                println!("{}", "Checkmate! White wins!".green().bold());
                game_active = false;
                break;
            } else if board.is_stalemate(false) {
                display_board(&mut board);
                println!("{}", "Stalemate! The game is a draw.".yellow().bold());
                game_active = false;
                break;
            }
            
            if board.b_in_check {
                println!("{}", "Black is in check!".red().bold());
            }
            
            let max_attempts = 100;
            let mut attempts = 0;
            let mut bot_move;
            let mut bot_move_valid = false;
            
            while attempts < max_attempts {
                bot_move = choose_bot_move(&mut board);
                
                if bot_move.contains("No valid moves available") {
                    if board.is_checkmate(false) {
                        display_board(&mut board);
                        println!("{}", "Checkmate! White wins!".green().bold());
                        game_active = false;
                    } else if board.is_stalemate(false) {
                        display_board(&mut board);
                        println!("{}", "Stalemate! The game is a draw.".yellow().bold());
                        game_active = false;
                    }
                    break;
                }
                
                bot_move_valid = parse_and_make_move(&mut board, &bot_move, false);
                if bot_move_valid {
                    break;
                }
                
                attempts += 1;
            }
            
            if attempts >= max_attempts {
                println!("{}", "Bot couldn't find a valid move. Game ending.".red().bold());
                game_active = false;
                break;
            }
            
            if bot_move_valid {
                if board.is_checkmate(true) {
                    display_board(&mut board);
                    println!("{}", "Checkmate! Black wins!".green().bold());
                    game_active = false;
                    break;
                } else if board.is_stalemate(true) {
                    display_board(&mut board);
                    println!("{}", "Stalemate! The game is a draw.".yellow().bold());
                    game_active = false;
                    break;
                }
            }
        } else {
            println!("{}", "Invalid move! Please try again.".red().bold());
        }
    }
    
    println!("{}", "Game Over!".blue().bold());
}