use super::super::position::Direction;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Pawn {
    position: Position,
    starting_position: bool,
}

impl Pawn {
    pub fn new(position: Position, starting_position: bool) -> Pawn {
        Pawn {
            position,
            starting_position,
        }
    }
}

impl Piece for Pawn {
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    fn icon(&self) -> Icon {
        Icon {
            dark: "♟",
            light: "♙",
        }
    }

    fn possible_moves(&self, board: &Board) -> Vec<Position> {
        let mut moves = Vec::new();
        let adjacent_pos = match self.position.move_copy(Direction::Up, 1) {
            Ok(pos) => pos,
            Err(_) => return moves,
        };

        if let Some(_) = board.collision(adjacent_pos) {
            return moves;
        }
        moves.push(adjacent_pos);

        let starting_move_pos = match self.position.move_copy(Direction::Up, 2) {
            Ok(pos) => pos,
            Err(_) => return moves,
        };
        if self.starting_position {
            if let None = board.collision(starting_move_pos) {
                moves.push(starting_move_pos);
            }
        }
        moves
    }

    fn possible_captures(&self, board: &Board) -> Vec<Position> {
        let mut captures = Vec::new();
        let capture_directions = vec![Direction::UpLeft, Direction::UpRight];
        for direction in capture_directions {
            if let Ok(capture_pos) = self.position.move_copy(direction, 1) {
                if let Some(_) = board.enemy_collision(capture_pos) {
                    captures.push(capture_pos);
                }
            }
        }
        captures
    }
}
