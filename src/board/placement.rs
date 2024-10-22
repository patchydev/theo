use crate::Board;
use crate::board::board::{Piece, Square, parse_position, Pawn};

impl Board {
    pub fn new() -> Self {
        let mut squares = [[Square { piece: None }; 8]; 8];

        for i in 0..8 {
            squares[1][i].piece = Some((Piece::P(Pawn { on_start: true, can_ep: false }), false));
            squares[6][i].piece = Some((Piece::P(Pawn { on_start: true, can_ep: false }), true));
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

        Board { squares }
    }

    pub fn generate_valid_moves(&self, color: bool) -> Vec<((usize, usize), (usize, usize))> {
        let mut valid_moves = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                if let Some((piece, piece_color)) = self.squares[i][j].piece {
                    if piece_color == color {
                        if let Piece::P(pawn) = piece {
                            let direction = if color { -1 } else { 1 };
                            let new_row = (i as isize + direction) as usize;

                            if pawn.on_start {
                                let row1 = (i as isize + direction) as usize;
                                let row2 = (i as isize + (direction + direction)) as usize;
                            }

                            if new_row < 8 && self.squares[new_row][j].piece.is_none() {
                                valid_moves.push(((i, j), (new_row, j)));
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
                    println!("Invalid move for queens!");
                    return false;
                }
            },

            Piece::K => {
                if (row_delta != col_delta) && ((row_delta != 0 && col_delta != 0) || (row_delta != 1 && col_delta != 1)) {
                    println!("Invalid move for kings!");
                    return false;
                }
            },
            
            _ => {
                
            }
        }

        board.squares[to.0][to.1].piece = board.squares[from.0][from.1].piece.take();
        return true;
    }

    false
}