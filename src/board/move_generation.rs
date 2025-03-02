use colored::Colorize;

use crate::Board;
use crate::board::board::Piece;
use crate::utils::board::choose_promotion_piece;

pub fn generate_valid_moves(board: &mut Board, color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_moves = Vec::new();

    for i in 0..8 {
        for j in 0..8 {
            if let Some((piece, piece_color)) = board.squares[i][j].piece {
                if piece_color == color {
                    match piece {
                        Piece::P => valid_moves.extend(generate_pawn_moves(board, (i, j), color)),
                        
                        Piece::B => valid_moves.extend(generate_bishop_moves(board, (i, j), color)),

                        Piece::R => valid_moves.extend(generate_rook_moves(board, (i, j), color)),

                        Piece::N => valid_moves.extend(generate_knight_moves(board, (i, j), color)),

                        Piece::Q => valid_moves.extend(generate_queen_or_king_moves(board, (i, j), color)),

                        Piece::K => valid_moves.extend(generate_queen_or_king_moves(board, (i, j), color)),
                    }
                }
            }
        }
    }

    valid_moves
}

fn generate_pawn_moves(board: &mut Board, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_moves = Vec::new();
    let direction = if color { -1 } else { 1 };
    let row1 = (from.0 as isize + direction) as usize;
    let row2 = (from.0 as isize + (direction + direction)) as usize;

    if row1 < 8 && board.squares[row1][from.1].piece.is_none() {
        valid_moves.push(((from.0, from.1), (row1, from.1)));
    }

    if (color && from.0 == 1) || (!color && from.0 == 6) {
        if row2 < 8 && board.squares[row1][from.1].piece.is_none() && board.squares[row2][from.1].piece.is_none() {
            valid_moves.push(((from.0, from.1), (row2, from.1)));
        }
    }

    let new_col_left = (from.1 as isize - 1) as usize;
    if row1 < 8 && new_col_left < 8 && board.squares[row1][new_col_left].piece.is_some() {
        let (_, captured_color) = board.squares[row1][new_col_left].piece.unwrap();
        if captured_color != color {
            valid_moves.push(((from.0, from.1), (row1, new_col_left)));
        }
    }

    let new_col_right = (from.1 as isize + 1) as usize;
    if row1 < 8 && new_col_right < 8 && board.squares[row1][new_col_right].piece.is_some() {
        let (_, captured_color) = board.squares[row1][new_col_right].piece.unwrap();
        if captured_color != color {
            valid_moves.push(((from.0, from.1), (row1, new_col_right)));
        }
    }

    if let Some(last_move) = board.last_move {
        if last_move.piece == Piece::P && last_move.color != color {
            if last_move.position.1 == from.1 {
                if last_move.position.0 as isize + direction == row1 as isize {
                    if new_col_left < 8 && board.squares[row1][new_col_left].piece.is_none() {
                        valid_moves.push(((from.0, from.1), (row1, new_col_left)));
                    }
                    if new_col_right < 8 && board.squares[row1][new_col_right].piece.is_none() {
                        valid_moves.push(((from.0, from.1), (row1, new_col_right)));
                    }
                }
            }
        }
    }

    if (color && row1 == 0) || (!color && row1 == 7) {
        valid_moves.push(((from.0, from.1), (row1, from.1)));
    }

    valid_moves
}

fn generate_queen_or_king_moves(board: &Board, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_moves = Vec::new();

    add_moves_in_direction(board, from, -1, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, -1, -1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, -1, color, &mut valid_moves);
    add_moves_in_direction(board, from, -1, 0, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, 0, color, &mut valid_moves);
    add_moves_in_direction(board, from, 0, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 0, -1, color, &mut valid_moves);

    valid_moves
}

fn generate_knight_moves(board: &Board, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_moves = Vec::new();

    add_moves_in_direction(board, from, -1, 2, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, 2, color, &mut valid_moves);
    add_moves_in_direction(board, from, 2, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 2, -1, color, &mut valid_moves);
    add_moves_in_direction(board, from, -1, -2, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, -2, color, &mut valid_moves);
    add_moves_in_direction(board, from, -2, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, -2, -1, color, &mut valid_moves);

    valid_moves
}

fn generate_rook_moves(board: &Board, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_moves = Vec::new();

    add_moves_in_direction(board, from, -1, 0, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, 0, color, &mut valid_moves);
    add_moves_in_direction(board, from, 0, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 0, -1, color, &mut valid_moves);

    valid_moves

}

fn generate_bishop_moves(board: &Board, from: (usize, usize), color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_moves = Vec::new();

    add_moves_in_direction(board, from, -1, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, -1, -1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, 1, color, &mut valid_moves);
    add_moves_in_direction(board, from, 1, -1, color, &mut valid_moves);

    valid_moves
}

fn add_moves_in_direction( // function to add moves in a given direction
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

pub fn gen_all_moves_for_color(board: &mut Board, color: bool) -> Vec<((usize, usize), (usize, usize))> {
    let mut moves = Vec::new();

    for row in 0..8 {
        for col in 0..8 {
            if let Some((piece, piece_color)) = board.squares[row][col].piece {
                if piece_color == color {
                    match piece {
                        Piece::P => {
                            moves.extend(generate_pawn_moves(board, (row, col), color));
                        },
                        Piece::N => {
                            moves.extend(generate_knight_moves(board, (row, col), color));
                        },
                        Piece::B => {
                            moves.extend(generate_bishop_moves(board, (row, col), color));
                        },
                        Piece::R => {
                            moves.extend(generate_rook_moves(board, (row, col), color));
                        },
                        Piece::Q | Piece::K => {
                            moves.extend(generate_queen_or_king_moves(board, (row, col), color));
                        },
                    }
                }
            }
        }
    }

    moves
}