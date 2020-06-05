use super::super::position::Direction;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Rook {
    position: Position,
}

impl Rook {
    fn new(position: Position) -> Self {
        Rook { position }
    }

    fn directions() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

impl Piece for Rook {
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Position> {
        Rook::directions()
            .iter()
            .map(|&direction| {
                let mut positions = vec![];
                for pos in walk_direction(self.position, direction) {
                    if let Some(_) = board.collision(pos) {
                        return positions;
                    }
                    positions.push(pos);
                }
                positions
            })
            .flatten()
            .collect()
    }
    fn possible_captures(&self, board: &Board) -> Vec<Position> {
        Rook::directions()
            .iter()
            .map(|&direction| {
                for pos in walk_direction(self.position, direction) {
                    if let Some(_) = board.enemy_collision(pos) {
                        return Some(pos);
                    }
                    if let Some(_) = board.collision(pos) {
                        return None;
                    }
                }
                None
            })
            .flatten() // this works because Option implements IntoIter, iterator over Some variants
            .collect()
    }
    fn icon(&self) -> Icon {
        Icon {
            dark: "♜",
            light: "♖",
        }
    }
}

#[test]
fn rook_possible_moves_empty() {
    let me1 = Rook::new(Position::new(3, 3));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![],
    };

    assert_eq!(
        vec![
            Position::new(3, 4),
            Position::new(3, 5),
            Position::new(3, 6),
            Position::new(3, 7),
            Position::new(4, 3),
            Position::new(5, 3),
            Position::new(6, 3),
            Position::new(7, 3),
            Position::new(3, 2),
            Position::new(3, 1),
            Position::new(3, 0),
            Position::new(2, 3),
            Position::new(1, 3),
            Position::new(0, 3),
        ],
        me1.possible_moves(&board),
        "rook moves on an empty board"
    );
}

#[test]
fn rook_possible_moves_collisions() {
    let me1 = Rook::new(Position::new(3, 3));
    let enemy1 = Rook::new(Position::new(4, 3));
    let enemy2 = Rook::new(Position::new(2, 3));
    let enemy3 = Rook::new(Position::new(3, 2));
    let enemy4 = Rook::new(Position::new(3, 4));
    let board = Board {
        my_pieces: vec![Box::new(me1)],
        enemy_pieces: vec![
            Box::new(enemy1),
            Box::new(enemy2),
            Box::new(enemy3),
            Box::new(enemy4),
        ],
    };

    let empty: Vec<Position> = vec![];
    assert_eq!(
        empty,
        me1.possible_moves(&board),
        "rook has collisions on every side"
    );
}

#[test]
fn rook_possible_moves_some_collisions() {
    let me1 = Rook::new(Position::new(3, 3));
    let me2 = Rook::new(Position::new(4, 3));
    let enemy2 = Rook::new(Position::new(2, 3));
    let enemy3 = Rook::new(Position::new(3, 2));
    let board = Board {
        my_pieces: vec![Box::new(me1), Box::new(me2)],
        enemy_pieces: vec![Box::new(enemy2), Box::new(enemy3)],
    };

    assert_eq!(
        vec![
            Position::new(3, 4),
            Position::new(3, 5),
            Position::new(3, 6),
            Position::new(3, 7)
        ],
        me1.possible_moves(&board),
        "rook has collisions on every side"
    );
}

#[test]
fn rook_possible_captures_some_collisions() {
    let me1 = Rook::new(Position::new(3, 3));
    let me2 = Rook::new(Position::new(4, 3));
    let enemy2 = Rook::new(Position::new(2, 3));
    let enemy3 = Rook::new(Position::new(3, 2));
    let enemy4 = Rook::new(Position::new(4, 2));
    let board = Board {
        my_pieces: vec![Box::new(me1), Box::new(me2)],
        enemy_pieces: vec![Box::new(enemy2), Box::new(enemy3), Box::new(enemy4)],
    };

    assert_eq!(
        vec![Position::new(3, 2), Position::new(2, 3)],
        me1.possible_captures(&board),
        "rook has 2 immidiate captures on every side"
    );
}

#[test]
fn rook_possible_captures_no_enemies() {
    let me1 = Rook::new(Position::new(3, 3));
    let me2 = Rook::new(Position::new(4, 3));
    let enemy1 = Rook::new(Position::new(0, 0));
    let board = Board {
        my_pieces: vec![Box::new(me1), Box::new(me2)],
        enemy_pieces: vec![Box::new(enemy1)],
    };

    let empty: Vec<Position> = vec![];
    assert_eq!(empty, me1.possible_captures(&board), "rook has no enemies");
}
