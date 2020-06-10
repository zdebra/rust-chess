use super::super::position::Direction;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct King {
    position: Position,
}

impl King {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    fn directions() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }
}

impl Piece for King {
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    fn icon(&self) -> Icon {
        Icon {
            dark: '♚',
            light: '♔',
        }
    }

    fn possible_moves(&self, board: &Board) -> Vec<Position> {
        King::directions()
            .iter()
            .map(|&direction| self.position.move_copy(direction, 1))
            .flatten()
            .filter(|&pos| board.collision(pos).is_none())
            // .filter(|&pos| !board.check_position(pos))
            .collect()
    }

    fn allowed_strike_destinations(&self, board: &Board) -> Vec<Position> {
        vec![]
    }
}

#[test]
fn possible_moves_empty_board() {
    let me1 = King::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![],
    };

    assert_eq!(
        vec![
            Position::new(3, 4),
            Position::new(4, 4),
            Position::new(4, 3),
            Position::new(4, 2),
            Position::new(3, 2),
            Position::new(2, 2),
            Position::new(2, 3),
            Position::new(2, 4),
        ],
        me1.possible_moves(&board),
        "king moves on an empty board"
    );
}

#[test]
fn possible_moves_allied_collisions() {
    let me1 = King::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![
            Box::new(me1),
            Box::new(Knight::new(Position::new(4, 4))),
            Box::new(Knight::new(Position::new(2, 3))),
        ],
        enemy_pieces: vec![],
    };

    assert_eq!(
        vec![
            Position::new(3, 4),
            Position::new(4, 3),
            Position::new(4, 2),
            Position::new(3, 2),
            Position::new(2, 2),
            Position::new(2, 4),
        ],
        me1.possible_moves(&board),
        "king moves on a board with collisions"
    );
}

#[test]
fn possible_moves_move_to_check() {
    let me1 = King::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![Box::new(Rook::new(Position::new(0, 4)))],
    };

    assert_eq!(
        vec![
            Position::new(4, 3),
            Position::new(4, 2),
            Position::new(3, 2),
            Position::new(2, 2),
            Position::new(2, 3),
        ],
        me1.possible_moves(&board),
        "king can't move to the check state"
    );
}
