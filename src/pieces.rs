use super::{board::Board, position::Direction, position::Position};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Piece {
    position: Position,
    starting_position: bool,
}

impl Piece {
    pub fn new(position: Position, starting_position: bool) -> Piece {
        Piece {
            position,
            starting_position,
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    fn possible_moves<'a>(&self, board: &Board) -> Vec<Position> {
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

    fn possible_captures<'a>(&self, board: &Board) -> Vec<Position> {
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

    pub fn possible_actions<'a>(&self, board: &Board) -> Vec<Position> {
        let mut actions = self.possible_moves(&board);
        actions.extend(self.possible_captures(&board));
        actions
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn swap_position(&mut self) {
        self.position.y = 7 - self.position.y
    }
}

pub fn swap_positions(pieces: &mut Vec<Piece>) {
    for piece in pieces.iter_mut() {
        piece.swap_position();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Action<'a> {
    pub piece: &'a Piece,
    pub destination: Position,
}

pub fn to_space<'a>(pieces: &'a Vec<Piece>) -> [Option<&'a Piece>; 64] {
    let mut board: [Option<&Piece>; 64] = [None; 64];
    for piece in pieces {
        let arr_pos = piece.position.arr_pos();
        board[arr_pos] = Some(piece);
    }
    board
}

#[test]
fn pawn_possible_captures() {
    let me2 = Piece::new(Position::new(7, 1), true);
    let me1 = Piece::new(Position::new(3, 1), true);
    let me3 = Piece::new(Position::new(1, 4), true);
    let me4 = Piece::new(Position::new(3, 4), true);
    let me5 = Piece::new(Position::new(0, 1), true);

    let enemy1 = Piece::new(Position::new(2, 2), false);
    let enemy2 = Piece::new(Position::new(4, 2), false);
    let enemy3 = Piece::new(Position::new(6, 2), false);
    let enemy4 = Piece::new(Position::new(4, 5), false);
    let enemy5 = Piece::new(Position::new(1, 2), false);
    let board = Board {
        my_pieces: vec![me1, me2, me3, me4, me5],
        enemy_pieces: vec![enemy1, enemy2, enemy3, enemy4, enemy5],
    };

    assert_eq!(
        vec![enemy1.position, enemy2.position],
        me1.possible_captures(&board),
        "pawn {} captures",
        me1.position
    );
    assert_eq!(
        vec![enemy3.position],
        me2.possible_captures(&board),
        "pawn {} captures on the right edge",
        me2.position
    );
    assert_eq!(
        vec![enemy5.position],
        me5.possible_captures(&board),
        "pawn {} captures on the left edge",
        me5.position
    );
    let empty_pos: Vec<Position> = Vec::new();
    assert_eq!(
        empty_pos,
        me3.possible_captures(&board),
        "pawn {} no targets",
        me3.position
    );
    assert_eq!(
        vec![enemy4.position],
        me4.possible_captures(&board),
        "pawn {} one target",
        me4.position
    );
}

#[test]
fn pawn_possible_moves() {
    let me1 = Piece::new(Position::new(0, 1), true);
    let me2 = Piece::new(Position::new(1, 1), true);
    let me3 = Piece::new(Position::new(2, 1), true);
    let me4 = Piece::new(Position::new(3, 2), false);
    let me5 = Piece::new(Position::new(4, 7), false);
    let enemy1 = Piece::new(Position::new(1, 2), false);
    let enemy2 = Piece::new(Position::new(2, 3), false);
    let board = Board {
        my_pieces: vec![me1, me2, me3, me4, me5],
        enemy_pieces: vec![enemy1, enemy2],
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
