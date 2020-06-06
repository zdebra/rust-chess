use super::super::position::Direction;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Knight {
    position: Position,
}

impl Knight {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    fn destinations(&self) -> Vec<Position> {
        vec![
            [(Direction::Up, 2), (Direction::Right, 1)],
            [(Direction::Up, 1), (Direction::Right, 2)],
            [(Direction::Right, 2), (Direction::Down, 1)],
            [(Direction::Right, 1), (Direction::Down, 2)],
            [(Direction::Down, 2), (Direction::Left, 1)],
            [(Direction::Down, 1), (Direction::Left, 2)],
            [(Direction::Left, 2), (Direction::Up, 1)],
            [(Direction::Left, 1), (Direction::Up, 2)],
        ]
        .iter()
        .map(|mv| {
            self.position
                .move_copy(mv[0].0, mv[0].1)
                .and_then(|x| x.move_copy(mv[1].0, mv[1].1))
        })
        .flatten()
        .collect()
    }
}

impl Piece for Knight {
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    fn icon(&self) -> Icon {
        Icon {
            dark: '♞',
            light: '♘',
        }
    }

    fn possible_moves(&self, board: &Board) -> Vec<Position> {
        self.destinations()
            .into_iter()
            .filter(|&x| board.collision(x).is_none())
            .collect()
    }

    fn possible_captures(&self, board: &Board) -> Vec<Position> {
        self.destinations()
            .into_iter()
            .filter(|&x| board.enemy_collision(x).is_some())
            .collect()
    }
}

#[test]
fn possible_moves_empty_board() {
    let me1 = Knight::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![],
    };

    assert_eq!(
        vec![
            Position::new(4, 5),
            Position::new(5, 4),
            Position::new(5, 2),
            Position::new(4, 1),
            Position::new(2, 1),
            Position::new(1, 2),
            Position::new(1, 4),
            Position::new(2, 5),
        ],
        me1.possible_moves(&board),
        "knight moves on an empty board"
    );
}

#[test]
fn possible_moves_collision() {
    let me1 = Knight::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![
            Box::new(me1),
            Box::new(Knight::new(Position::new(4, 5))),
            Box::new(Knight::new(Position::new(1, 4))),
        ],
        enemy_pieces: vec![
            Box::new(Knight::new(Position::new(1, 2))),
            Box::new(Knight::new(Position::new(2, 3))),
            Box::new(Knight::new(Position::new(2, 2))),
            Box::new(Knight::new(Position::new(3, 2))),
        ],
    };

    assert_eq!(
        vec![
            Position::new(5, 4),
            Position::new(5, 2),
            Position::new(4, 1),
            Position::new(2, 1),
            Position::new(2, 5),
        ],
        me1.possible_moves(&board),
        "knight moves with collisions"
    );
}

#[test]
fn possible_moves_corner() {
    let me1 = Knight::new(Position::new(7, 0));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![],
    };

    assert_eq!(
        vec![Position::new(5, 1), Position::new(6, 2),],
        me1.possible_moves(&board),
        "knight moves from the corner"
    );
}

#[test]
fn possible_captures_collision() {
    let me1 = Knight::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![
            Box::new(me1),
            Box::new(Knight::new(Position::new(4, 5))),
            Box::new(Knight::new(Position::new(1, 4))),
        ],
        enemy_pieces: vec![
            Box::new(Knight::new(Position::new(1, 2))),
            Box::new(Knight::new(Position::new(2, 3))),
            Box::new(Knight::new(Position::new(2, 2))),
            Box::new(Knight::new(Position::new(3, 2))),
        ],
    };

    assert_eq!(
        vec![Position::new(1, 2)],
        me1.possible_captures(&board),
        "knight captures"
    );
}
