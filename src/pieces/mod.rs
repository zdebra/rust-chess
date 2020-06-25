use super::position::{Direction, Position};
use super::ray::Ray;

pub enum Piece {
    Pawn(Position),
}

impl Piece {
    pub fn legal_moves(&self) -> Vec<Ray> {
        match self {
            Piece::Pawn(pos) => pawn_moves(*pos),
        }
    }
    pub fn collides_with(&self, position: Position) -> bool {
        match self {
            Piece::Pawn(pos) => &position == pos,
        }
    }
}

fn pawn_moves(position: Position) -> Vec<Ray> {
    let starting_pos = match position.move_copy(Direction::Up, 1) {
        Ok(pos) => pos,
        Err(_) => return vec![],
    };

    let mut ray_limit = 1;
    if position.y == 1 {
        if let Ok(_) = position.move_copy(Direction::Up, 2) {
            ray_limit = 2;
        }
    }
    vec![Ray::new(starting_pos, Direction::Up, ray_limit)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pawn_legal_moves() {
        let pawn = Piece::Pawn(Position::new(3, 3));
        assert_eq!(
            vec![Ray::new_from(3, 4, Direction::Up, 1)],
            pawn.legal_moves()
        );
        let pawn_starting = Piece::Pawn(Position::new(4, 1));
        assert_eq!(
            vec![Ray::new_from(4, 2, Direction::Up, 2)],
            pawn_starting.legal_moves()
        );
    }
}
