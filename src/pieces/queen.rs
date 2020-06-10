use super::super::position::Direction;
use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Queen {
    position: Position,
}

impl Queen {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    fn directions() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::UpRight,
            Direction::DownRight,
            Direction::DownLeft,
            Direction::UpLeft,
        ]
    }
}

impl Piece for Queen {
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Position> {
        linear_moves(Queen::directions().iter(), self.position, board)
    }

    fn allowed_strike_destinations(&self, board: &Board) -> Vec<Position> {
        linear_raw_captures(Self::directions().iter(), self.position, board)
    }
    fn icon(&self) -> Icon {
        Icon {
            dark: '♛',
            light: '♕',
        }
    }
}
