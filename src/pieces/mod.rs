use super::{board::Board, position::Direction, position::Position};
pub mod pawn;
pub use pawn::Pawn;
pub mod rook;
pub use rook::Rook;
pub mod bishop;
pub use bishop::Bishop;
pub mod queen;
pub use queen::Queen;
pub mod knight;
pub use knight::Knight;
pub mod king;
pub use king::King;

pub trait Piece: std::fmt::Debug {
    fn get_position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn possible_moves(&self, board: &Board) -> Vec<Position>;

    /// Reports all legal capture moves that piece can do from its current position.
    /// It doesn't matter whether there is a piece or not.
    fn allowed_strike_destinations(&self, board: &Board) -> Vec<Position>;
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

    fn possible_captures(&self, board: &Board) -> Vec<Position> {
        self.allowed_strike_destinations(board)
            .into_iter()
            .filter(|&pos| board.enemy_collision(pos).is_some())
            .collect()
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
    pub dark: char,
    pub light: char,
}

fn walk_direction(cur_position: Position, direction: Direction) -> Vec<Position> {
    let mut output = vec![];
    let mut pos = cur_position;
    while let Ok(next_pos) = pos.move_copy(direction, 1) {
        output.push(next_pos);
        pos = next_pos;
    }
    output
}

fn linear_moves<'a, T>(directions: T, cur_pos: Position, board: &Board) -> Vec<Position>
where
    T: Iterator<Item = &'a Direction>,
{
    directions
        .map(|&direction| {
            let mut positions = vec![];
            for pos in walk_direction(cur_pos, direction) {
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

fn linear_captures<'a, T>(directions: T, cur_pos: Position, board: &Board) -> Vec<Position>
where
    T: Iterator<Item = &'a Direction>,
{
    directions
        .map(|&direction| {
            for pos in walk_direction(cur_pos, direction) {
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

fn linear_raw_captures<'a, T>(directions: T, cur_pos: Position, board: &Board) -> Vec<Position>
where
    T: Iterator<Item = &'a Direction>,
{
    directions
        .map(|&direction| {
            let mut output = vec![];
            for pos in walk_direction(cur_pos, direction) {
                if let Some(_) = board.my_collision(pos) {
                    break;
                }
                output.push(pos);
                if let Some(_) = board.enemy_collision(pos) {
                    break;
                }
            }
            output
        })
        .flatten() // this works because Option implements IntoIter, iterator over Some variants
        .collect()
}

#[test]
fn walk_direction_bottom_up_all_the_way() {
    let walked = walk_direction(Position::new(0, 0), Direction::Up);
    assert_eq!(
        vec![
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(0, 3),
            Position::new(0, 4),
            Position::new(0, 5),
            Position::new(0, 6),
            Position::new(0, 7)
        ],
        walked
    );
}

#[test]
fn walk_direction_up() {
    let walked = walk_direction(Position::new(2, 2), Direction::Up);
    assert_eq!(
        vec![
            Position::new(2, 3),
            Position::new(2, 4),
            Position::new(2, 5),
            Position::new(2, 6),
            Position::new(2, 7),
        ],
        walked
    );
}

#[test]
fn walk_direction_empty() {
    let walked = walk_direction(Position::new(2, 7), Direction::Up);
    let empty: Vec<Position> = vec![];
    assert_eq!(empty, walked);
}
