use super::super::position::Direction;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Bishop {
    position: Position,
}

impl Bishop {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    fn directions() -> [Direction; 4] {
        [
            Direction::UpRight,
            Direction::DownRight,
            Direction::DownLeft,
            Direction::UpLeft,
        ]
    }
}

impl Piece for Bishop {
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Position> {
        linear_moves(Bishop::directions().iter(), self.position, board)
    }
    fn possible_captures(&self, board: &Board) -> Vec<Position> {
        linear_captures(Bishop::directions().iter(), self.position, board)
    }
    fn icon(&self) -> Icon {
        Icon {
            dark: "♝",
            light: "♙",
        }
    }
}

#[test]
fn bishop_possible_moves_empty() {
    let me1 = Bishop::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![],
    };

    assert_eq!(
        vec![
            Position::new(4, 4),
            Position::new(5, 5),
            Position::new(6, 6),
            Position::new(7, 7),
            Position::new(4, 2),
            Position::new(5, 1),
            Position::new(6, 0),
            Position::new(2, 2),
            Position::new(1, 1),
            Position::new(0, 0),
            Position::new(2, 4),
            Position::new(1, 5),
            Position::new(0, 6),
        ],
        me1.possible_moves(&board),
        "bishop moves on an empty board"
    );
}
