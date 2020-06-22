use super::position::{Direction, Position};

enum Piece {
    Pawn(Position),
}

impl Piece {
    fn legal_moves(&self) -> Vec<Ray> {
        match self {
            Piece::Pawn(pos) => pawn_moves(*pos),
        }
    }
}

fn pawn_moves(position: Position) -> Vec<Ray> {
    let starting_pos = match position.move_copy(Direction::Up, 1) {
        Ok(pos) => pos,
        Err(_) => return vec![],
    };

    let mut ray = Ray {
        direction: Direction::Up,
        start: starting_pos,
        limit: 1,
    };
    if position.y == 1 {
        if let Ok(_) = position.move_copy(Direction::Up, 2) {
            ray.limit = 2;
        }
    }
    vec![ray]
}

struct Board {
    my_pieces: Vec<Piece>,
    enemy_pieces: Vec<Piece>,
}

impl Board {
    fn legal_moves(&self) -> Vec<Action> {
        self.my_pieces
            .iter()
            .flat_map(|my_piece| my_piece.legal_moves())
            .flat_map(|ray| ray.move_actions(&self.my_pieces, &self.enemy_pieces))
            .collect()
    }
}

pub struct Action {
    pub source: Position,
    pub destination: Position,
}

#[derive(std::fmt::Debug, PartialEq, Eq)]
struct Ray {
    start: Position,
    direction: Direction,
    limit: usize,
}

impl Ray {
    /// Yields Vec of Actions that
    fn move_actions(&self, my_pieces: &Vec<Piece>, enemy_pieces: &Vec<Piece>) -> Vec<Action> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn piece_legal_moves() {
        let pawn = Piece::Pawn(Position::new(3, 3));
        assert_eq!(
            vec![Ray {
                start: Position::new(3, 4),
                direction: Direction::Up,
                limit: 1
            }],
            pawn.legal_moves()
        );
    }
}
