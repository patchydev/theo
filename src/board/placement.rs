use crate::Board;
use crate::board::board::{Piece, Square};
use crate::utils::board::{choose_promotion_piece, parse_position};

impl Board {
    pub fn new() -> Self {
        let mut squares = [[Square { piece: None }; 8]; 8];

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

        Board { squares, last_move: None }
    }

    pub fn generate_valid_moves(&mut self, color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                if let Some((piece, piece_color)) = self.squares[i][j].piece {
                    if piece_color == color {
                        if let Piece::P = piece {
                            let direction = if color { -1 } else { 1 };
                            let row1 = (i as isize + direction) as usize;
                            let row2 = (i as isize + (direction + direction)) as usize;

                            if row1 < 8 && self.squares[row1][j].piece.is_none() {
                                valid_moves.push(((i, j), (row1, j)));
                            }

                            if (color && i == 1) || (!color && i == 6) {
                                if row2 < 8 && self.squares[row1][j].piece.is_none() && self.squares[row2][j].piece.is_none() {
                                    valid_moves.push(((i, j), (row2, j)));
                                }
                            }

                            let new_col_left = (j as isize - 1) as usize;
                            if row1 < 8 && new_col_left < 8 && self.squares[row1][new_col_left].piece.is_some() {
                                let (_, captured_color) = self.squares[row1][new_col_left].piece.unwrap();
                                if captured_color != color {
                                    valid_moves.push(((i, j), (row1, new_col_left)));
                                }
                            }
                        
                            let new_col_right = (j as isize + 1) as usize;
                            if row1 < 8 && new_col_right < 8 && self.squares[row1][new_col_right].piece.is_some() {
                                let (_, captured_color) = self.squares[row1][new_col_right].piece.unwrap();
                                if captured_color != color {
                                    valid_moves.push(((i, j), (row1, new_col_right)));
                                }
                            }

                            if let Some(last_move) = self.last_move {
                                if last_move.piece == Piece::P && last_move.color != color {
                                    if last_move.position.1 == j {
                                        if last_move.position.0 as isize + direction == row1 as isize {
                                            if new_col_left < 8 && self.squares[row1][new_col_left].piece.is_none() {
                                                valid_moves.push(((i, j), (row1, new_col_left)));
                                            }
                                            if new_col_right < 8 && self.squares[row1][new_col_right].piece.is_none() {
                                                valid_moves.push(((i, j), (row1, new_col_right)));
                                            }
                                        }
                                    }
                                }
                            }

                            if row1 == 0 || row1 == 7 {
                                let promotion_piece = choose_promotion_piece();

                                if promotion_piece.is_ok() {
                                    self.squares[row1][j].piece = Some((promotion_piece.unwrap(), color));
                                } else {
                                    println!("Error while promoting: {}", promotion_piece.unwrap_err());
                                }
                            }
                        
                        }

                        else if piece == Piece::B {
                            valid_moves.extend(self.generate_bishop_moves((i, j), color));
                        }

                        else if piece == Piece::R {
                            valid_moves.extend(self.generate_rook_moves((i, j), color));
                        }

                        else if piece == Piece::N {
                            valid_moves.extend(self.generate_knight_moves((i, j), color));
                        }

                        else if piece == Piece::Q {
                            valid_moves.extend(self.generate_queen_moves((i, j), color));
                        }

                        else if piece == Piece::K {
                            valid_moves.extend(self.generate_king_moves((i, j), color));
                        }

                        else {
                            println!("Can't move that piece yet! Sorry :(");
                        }
                    }
                }
            }
        }

        valid_moves
    }

    fn generate_king_moves(&self, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        self.add_moves_in_direction(self, from, -1, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -1, -1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, -1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -1, 0, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 0, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 0, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 0, -1, color, &mut valid_moves);

        valid_moves
    }

    fn generate_queen_moves(&self, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        self.add_moves_in_direction(self, from, -1, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -1, -1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, -1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -1, 0, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 0, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 0, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 0, -1, color, &mut valid_moves);

        valid_moves
    }

    fn generate_knight_moves(&self, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        self.add_moves_in_direction(self, from, -1, 2, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 2, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 2, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 2, -1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -1, -2, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, -2, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -2, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -2, -1, color, &mut valid_moves);

        valid_moves
    }

    fn generate_rook_moves(&self, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        self.add_moves_in_direction(self, from, -1, 0, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 0, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 0, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 0, -1, color, &mut valid_moves);

        valid_moves

    }

    fn generate_bishop_moves(&self, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        self.add_moves_in_direction(self, from, -1, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, -1, -1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, 1, color, &mut valid_moves);
        self.add_moves_in_direction(self, from, 1, -1, color, &mut valid_moves);

        valid_moves
    }

    fn add_moves_in_direction(
        &self,
        board: &Board,
        from: (usize, usize),
        row_delta: isize,
        col_delta: isize,
        color: bool,
        valid_moves: &mut Vec<((usize, usize), (usize, usize))>
    ) {
        let (mut row, mut col) = (from.0 as isize, from.1 as isize);

        loop {
            row += row_delta;
            col += col_delta;

            if row < 0 || row >= 8 || col < 0 || col >= 8 {
                break;
            }

            let to = (row as usize, col as usize);
            if let Some((_, piece_color)) = board.squares[to.0][to.1].piece {
                if piece_color != color {
                    valid_moves.push((from, to));
                }
                break;
            } else {
                valid_moves.push((from, to));
            }
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

        let row_delta = (to.0 as isize - from.0 as isize).abs();
        let col_delta = (to.1 as isize - from.1 as isize).abs();

        match piece {
            Piece::B => {
                if row_delta != col_delta {
                    println!("Invalid move for Bishop! Bishops must move diagonally.");
                    return false;
                }
            },

            Piece::R => {
                if row_delta != 0 && col_delta != 0 {
                    println!("Invalid move for Rook! Rooks must move either horizontally or vertically.");
                    return false;
                }
            },

            Piece::N => {
                if !((row_delta == 2 && col_delta == 1) || (row_delta == 1 && col_delta == 2)) {
                    println!("Invalid move for Knight! Knights must move in an L shape.");
                    return false;
                }
            },

            Piece::Q => {
                if (row_delta != col_delta) && (row_delta != 0 && col_delta != 0) {
                    println!("Invalid move for queens! queens are a combination of rooks and bishops");
                    return false;
                }
            },

            Piece::K => {
                if (row_delta != col_delta) && ((row_delta != 0 && col_delta != 0) || (row_delta != 1 && col_delta != 1)) {
                    println!("Invalid move for kings! kings move the same as queens but only one square");
                    return false;
                }
            },

            Piece::P => {
                let direction = if color { -1 } else { 1 };

                if to.1 == from.1 {
                    if to.0 as isize == from.0 as isize + direction {
                        if board.squares[to.0][to.1].piece.is_none() {
                            if to.0 == 0 || to.0 == 7 {
                                board.squares[to.0][to.1].piece = Some((Piece::Q, color));
                            } else {
                                board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                            }
                            if to.0 == 0 || to.0 == 7 {
                                let promotion_piece = choose_promotion_piece();

                                if promotion_piece.is_ok() {
                                    board.squares[to.0][to.1].piece = Some((promotion_piece.unwrap(), color));
                                } else {
                                    println!("Error while promoting: {}", promotion_piece.unwrap_err());
                                }
                            }
                            return true;
                        }
                    }
            
                    if (!color && from.0 == 1) || (color && from.0 == 6) {
                        if to.0 as isize == from.0 as isize + (2 * direction) {
                            let intermediate_row = (from.0 as isize + direction) as usize;
                            if board.squares[to.0][to.1].piece.is_none() && 
                               board.squares[intermediate_row][to.1].piece.is_none() {
                                board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                                return true;
                            }
                        }
                    }
                    
                }
            
                if row_delta == 1 && col_delta == 1 {
                    if let Some((_, captured_color)) = board.squares[to.0][to.1].piece {
                        if captured_color != color {
                            board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                            if to.0 == 0 || to.0 == 7 {
                                let promotion_piece = choose_promotion_piece();

                                if promotion_piece.is_ok() {
                                    board.squares[to.0][to.1].piece = Some((promotion_piece.unwrap(), color));
                                } else {
                                    println!("Error while promoting: {}", promotion_piece.unwrap_err());
                                }
                            }
                            return true;
                        }
                    }
                }
            
                if col_delta == 1 && row_delta == 1 {
                    if let Some(last_move) = &board.last_move {
                        if last_move.piece == Piece::P && last_move.color != color {
                            if (to.0 as isize) == (last_move.position.0 as isize + direction) && 
                               to.1 == last_move.position.1 {
                                board.squares[last_move.position.0][last_move.position.1].piece = None;
                                board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
                                return true;
                            }
                        }
                    }
                }
            
                println!("Invalid movement for Pawn!");
                return false;
            },
        }

        board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
        return true;
    }

    false
}