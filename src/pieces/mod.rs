use super::{board::Board, position::Position};
pub mod pawn;
pub use pawn::Pawn;

pub trait Piece: std::fmt::Debug {
    fn get_position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn possible_moves(&self, board: &Board) -> Vec<Position>;
    fn possible_captures(&self, board: &Board) -> Vec<Position>;
    fn icon(&self) -> Icon;

    fn possible_actions(&self, board: &Board) -> Vec<Position> {
        let mut actions = self.possible_moves(&board);
        actions.extend(self.possible_captures(&board));
        actions
    }

    fn swap_position(&mut self) {
        let pos = self.get_position();
        self.set_position(Position::new(pos.x, 7 - pos.y));
    }
}

impl PartialEq for dyn Piece {
    fn eq(&self, other: &Self) -> bool {
        self.get_position() == other.get_position()
    }
}
impl Eq for dyn Piece {}

pub fn swap_positions(pieces: &mut Vec<Box<dyn Piece>>) {
    for piece in pieces.iter_mut() {
        piece.swap_position();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Action {
    pub source: Position,
    pub destination: Position,
}

pub fn to_space<'a>(pieces: &'a Vec<Box<dyn Piece>>) -> [Option<&'a Box<dyn Piece>>; 64] {
    let mut board: [Option<&Box<dyn Piece>>; 64] = [None; 64];
    for piece in pieces {
        let arr_pos = piece.get_position().arr_pos();
        board[arr_pos] = Some(piece);
    }
    board
}

pub struct Icon {
    pub dark: &'static str,
    pub light: &'static str,
}
