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
