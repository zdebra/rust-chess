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

#[test]
fn pawn_possible_captures() {
    let me1 = Pawn::new(Position::new(3, 1), true);
    let me2 = Pawn::new(Position::new(7, 1), true);
    let me3 = Pawn::new(Position::new(1, 4), true);
    let me4 = Pawn::new(Position::new(3, 4), true);
    let me5 = Pawn::new(Position::new(0, 1), true);

    let enemy1 = Pawn::new(Position::new(2, 2), false);
    let enemy2 = Pawn::new(Position::new(4, 2), false);
    let enemy3 = Pawn::new(Position::new(6, 2), false);
    let enemy4 = Pawn::new(Position::new(4, 5), false);
    let enemy5 = Pawn::new(Position::new(1, 2), false);
    let board = Board {
        my_pieces: vec![
            Box::new(me1),
            Box::new(me2),
            Box::new(me3),
            Box::new(me4),
            Box::new(me5),
        ],
        enemy_pieces: vec![
            Box::new(enemy1),
            Box::new(enemy2),
            Box::new(enemy3),
            Box::new(enemy4),
            Box::new(enemy5),
        ],
    };

    assert_eq!(
        vec![enemy1.get_position(), enemy2.get_position()],
        me1.possible_captures(&board),
        "pawn {} captures",
        me1.get_position()
    );
    assert_eq!(
        vec![enemy3.get_position()],
        me2.possible_captures(&board),
        "pawn {} captures on the right edge",
        me2.get_position()
    );
    assert_eq!(
        vec![enemy5.get_position()],
        me5.possible_captures(&board),
        "pawn {} captures on the left edge",
        me5.get_position()
    );
    let empty_pos: Vec<Position> = Vec::new();
    assert_eq!(
        empty_pos,
        me3.possible_captures(&board),
        "pawn {} no targets",
        me3.get_position()
    );
    assert_eq!(
        vec![enemy4.get_position()],
        me4.possible_captures(&board),
        "pawn {} one target",
        me4.get_position()
    );
}

#[test]
fn pawn_possible_moves() {
    let me1 = Pawn::new(Position::new(0, 1), true);
    let me2 = Pawn::new(Position::new(1, 1), true);
    let me3 = Pawn::new(Position::new(2, 1), true);
    let me4 = Pawn::new(Position::new(3, 2), false);
    let me5 = Pawn::new(Position::new(4, 7), false);
    let enemy1 = Pawn::new(Position::new(1, 2), false);
    let enemy2 = Pawn::new(Position::new(2, 3), false);
    let board = Board {
        my_pieces: vec![
            Box::new(me1),
            Box::new(me2),
            Box::new(me3),
            Box::new(me4),
            Box::new(me5),
        ],
        enemy_pieces: vec![Box::new(enemy1), Box::new(enemy2)],
    };

    assert_eq!(
        vec![Position::new(0, 2), Position::new(0, 3)],
        me1.possible_moves(&board),
        "pawn moves from starting position"
    );
    assert_eq!(
        vec![Position::new(3, 3)],
        me4.possible_moves(&board),
        "pawn moves"
    );
    let empty_pos: Vec<Position> = Vec::new();
    assert_eq!(
        empty_pos,
        me5.possible_moves(&board),
        "pawn moves out of board"
    );
    assert_eq!(
        empty_pos,
        me2.possible_moves(&board),
        "pawn moves from starting position with direct collision"
    );
    assert_eq!(
        vec![Position::new(2, 2)],
        me3.possible_moves(&board),
        "pawn moves from starting position with collision"
    );
}
