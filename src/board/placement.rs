use colored::Colorize;

use crate::Board;
use crate::board::{board::{Piece, Square}, move_generation::gen_all_moves_for_color};
use crate::utils::board::{choose_promotion_piece, parse_position};

impl Board {
    pub fn new() -> Self { // setup board
        let mut squares = [[Square { piece: None }; 8]; 8];

        let mut b_in_check = false;
        let mut w_in_check = false;

        let mut w_king_pos = (0, 4);
        let mut b_king_pos = (7, 4);

        for i in 0..8 {
            squares[1][i].piece = Some((Piece::P, false));
            squares[6][i].piece = Some((Piece::P, true));
        }

        squares[0][0].piece = Some((Piece::R, false));
        squares[0][1].piece = Some((Piece::N, false));
        squares[0][2].piece = Some((Piece::B, false));
        squares[0][3].piece = Some((Piece::Q, false));
        squares[0][4].piece = Some((Piece::K, false));
        squares[0][5].piece = Some((Piece::B, false));
        squares[0][6].piece = Some((Piece::N, false));
        squares[0][7].piece = Some((Piece::R, false));

        squares[7][0].piece = Some((Piece::R, true));
        squares[7][1].piece = Some((Piece::N, true));
        squares[7][2].piece = Some((Piece::B, true));
        squares[7][3].piece = Some((Piece::Q, true));
        squares[7][4].piece = Some((Piece::K, true));
        squares[7][5].piece = Some((Piece::B, true));
        squares[7][6].piece = Some((Piece::N, true));
        squares[7][7].piece = Some((Piece::R, true));

        Board { squares, last_move: None, w_king_pos, b_king_pos, w_in_check, b_in_check }
    }

    fn update_check_status(&mut self, color: bool) {
        let king_position = if !color { self.w_king_pos } else { self.b_king_pos };
        
        let opponent_moves = gen_all_moves_for_color(self, !color);

        let in_check = opponent_moves.iter().any(|&(_, destination)| destination == king_position);

        if !color {
            self.w_in_check = in_check;
        } else {
            self.b_in_check = in_check;
        }
    }
}

pub fn parse_and_make_move(board: &mut Board, move_str: &str, color: bool) -> bool {
    let parts: Vec<&str> = move_str.split_whitespace().collect();
    if parts.len() != 2 {
        return false;
    }

    let from = parse_position(parts[0]);
    let to = parse_position(parts[1]);

    if let Some((piece, piece_color)) = board.squares[from.0][from.1].piece {
        if piece_color != color {
            return false;
        }

        fn check_if_capturing_own_piece(board: &mut Board, to: (usize, usize), piece_color: bool) -> bool {
            if !(board.squares[to.0][to.1].piece.is_none()) {
                if piece_color == board.squares[to.0][to.1].piece.unwrap().1 {
                    return true;
                }
            }
            return false;
        }

        let row_delta = (to.0 as isize - from.0 as isize).abs();
        let col_delta = (to.1 as isize - from.1 as isize).abs();

        match piece {
            Piece::B => {
                if row_delta != col_delta { // rows and cols must be equal to move diagonally
                    println!("{}", "Invalid move for Bishop! Bishops must move diagonally.".red().bold());
                    return false;
                }

                if check_if_capturing_own_piece(board, to, piece_color) { return false; }
            },

            Piece::R => {
                if row_delta != 0 && col_delta != 0 { // I don't know why it's && and not || but it works
                    println!("{}", "Invalid move for Rook! Rooks must move either horizontally or vertically.".red().bold());
                    return false;
                }

                if check_if_capturing_own_piece(board, to, piece_color) { return false; }
            },

            Piece::N => {
                if !((row_delta == 2 && col_delta == 1) || (row_delta == 1 && col_delta == 2)) { // L shape
                    println!("{}", "Invalid move for Knight! Knights must move in an L shape.".red().bold());
                    return false;
                }

                if check_if_capturing_own_piece(board, to, piece_color) { return false; }
            },

            Piece::Q => {
                if (row_delta != col_delta) && (row_delta != 0 && col_delta != 0) { // combination of rooks and bishops
                    println!("{}", "Invalid move for queens! queens are a combination of rooks and bishops".red().bold());
                    return false;
                }

                if check_if_capturing_own_piece(board, to, piece_color) { return false; }
            },

            Piece::K => {
                if row_delta > 1 || col_delta > 1 { // limit kings to one square
                    println!("{}", "Invalid move for kings! kings move the same as queens but only one square".red().bold());
                    return false;
                }
            },

            Piece::P => {
                let direction = if color { -1 } else { 1 };

                if to.1 == from.1 { // going in a straight line
                    if to.0 as isize == from.0 as isize + direction { // going one square
                        if board.squares[to.0][to.1].piece.is_none() {
                            if to.0 == 0 || to.0 == 8 { // are we promoting?
                                let promotion_piece = choose_promotion_piece();

                                if promotion_piece.is_ok() {
                                    board.squares[to.0][to.1].piece = Some((promotion_piece.unwrap(), color));
                                } else {
                                    println!("{} {}", "Error while promoting:".red().bold(), promotion_piece.unwrap_err());
                                }
                            } else { // if not, just move the pawn
                                board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                            }
                            board.update_check_status(color);
                            return true;
                        }
                    }
            
                    if (!color && from.0 == 1) || (color && from.0 == 6) { // going two squares, this should not be like this but it works
                        if to.0 as isize == from.0 as isize + (2 * direction) {
                            let intermediate_row = (from.0 as isize + direction) as usize;
                            if board.squares[to.0][to.1].piece.is_none() && 
                               board.squares[intermediate_row][to.1].piece.is_none() {
                                board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                                board.update_check_status(color);
                                return true;
                            }
                        }
                    }
                    
                }
            
                if row_delta == 1 && col_delta == 1 { // capturing
                    if let Some((_, captured_color)) = board.squares[to.0][to.1].piece {
                        if captured_color != color {
                            board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                            if to.0 == 0 || to.0 == 8 {
                                let promotion_piece = choose_promotion_piece();

                                if promotion_piece.is_ok() {
                                    board.squares[to.0][to.1].piece = Some((promotion_piece.unwrap(), color));
                                } else {
                                    println!("{} {}", "Error while promoting:".red().bold(), promotion_piece.unwrap_err());
                                }
                            }
                            board.update_check_status(color);
                            return true;
                        }
                    }
                }
            
                if col_delta == 1 && row_delta == 1 { // en passant
                    if let Some(last_move) = &board.last_move {
                        if last_move.piece == Piece::P && last_move.color != color {
                            if (to.0 as isize) == (last_move.position.0 as isize + direction) && 
                               to.1 == last_move.position.1 {
                                board.squares[last_move.position.0][last_move.position.1].piece = None;
                                board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                                board.update_check_status(color);
                                return true;
                            }
                        }
                    }
                }
            
                println!("{}", "Invalid movement for Pawn!".red().bold());
                return false;
            },
        }

        board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take(); // move any piece that is not a pawn
        board.update_check_status(color);
        return true;
    }

    false
}