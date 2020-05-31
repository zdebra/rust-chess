use rand::seq::SliceRandom;
use std::fmt;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("position out of bounds")]
    PositionOutOfBounds,
    #[error("action is invalid")]
    InvalidAction,
}

struct Board {
    my_pieces: Vec<Piece>,
    enemy_pieces: Vec<Piece>,
}

impl fmt::Display for Board {
    // https://chess.stackexchange.com/questions/1600/chess-program-for-linux-unix-console
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let my_pieces_space = to_space(&self.my_pieces);
        let enemy_pieces_space = to_space(&self.enemy_pieces);

        let board_strs: Vec<String> = my_pieces_space
            .iter()
            .zip(enemy_pieces_space.iter())
            .rev()
            .map(|(my_place, enemy_place)| match (my_place, enemy_place) {
                (Some(piece), None) => "♙".to_string(),
                (None, Some(piece)) => "♟".to_string(),
                _ => ".".to_string(),
            })
            .collect();

        f.write_str("  a b c d e f g h\n")?;
        for (row_index, row) in board_strs.chunks(8).enumerate() {
            f.write_fmt(format_args!(
                "{} {} {} {} {} {} {} {} {} {}\n",
                8 - row_index,
                row[7],
                row[6],
                row[5],
                row[4],
                row[3],
                row[2],
                row[1],
                row[0],
                8 - row_index,
            ))?;
        }
        f.write_str("  a b c d e f g h\n")?;
        Ok(())
    }
}

impl Board {
    fn my_collision(&self, position: &Position) -> Option<&Piece> {
        for piece in self.my_pieces.iter() {
            if &piece.position == position {
                return Some(piece);
            }
        }
        None
    }
    fn enemy_collision(&self, position: &Position) -> Option<&Piece> {
        for piece in self.enemy_pieces.iter() {
            if &piece.position == position {
                return Some(piece);
            }
        }
        None
    }
    fn collision(&self, position: &Position) -> Option<&Piece> {
        if let Some(my_piece) = self.my_collision(position) {
            Some(my_piece)
        } else if let Some(enemy_piece) = self.enemy_collision(position) {
            Some(enemy_piece)
        } else {
            None
        }
    }

    fn possible_actions(&self) -> Vec<PieceAction> {
        let mut actions = Vec::new();
        for piece in self.my_pieces.iter() {
            for action in piece.possible_actions(&self) {
                actions.push(PieceAction {
                    piece: piece,
                    destination: action,
                });
            }
        }
        actions
    }

    fn swap_sides(&mut self) {
        std::mem::swap(&mut self.my_pieces, &mut self.enemy_pieces);
    }

    fn play(&self, action: &PieceAction) -> Result<Board, Error> {
        if !self.possible_actions().iter().any(|a| a == action) {
            return Err(Error::InvalidAction);
        }

        // TODO: get rid of clone
        let mut output_board = Board {
            my_pieces: self.my_pieces.clone(),
            enemy_pieces: self.enemy_pieces.clone(),
        };

        if let Some(enemy_piece) = self.enemy_collision(&action.destination) {
            let enemy_piece_index = self
                .enemy_pieces
                .iter()
                .position(|piece| piece == enemy_piece)
                .unwrap();
            output_board.enemy_pieces.remove(enemy_piece_index);
        }

        output_board
            .my_pieces
            .iter_mut()
            .find(|piece| piece == &action.piece)
            .unwrap()
            .set_position(action.destination);

        output_board.swap_sides();
        Ok(output_board)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PieceAction<'a> {
    piece: &'a Piece,
    destination: Position,
}

fn to_space<'a>(pieces: &'a Vec<Piece>) -> [Option<&'a Piece>; 64] {
    let mut board: [Option<&Piece>; 64] = [None; 64];
    for piece in pieces {
        let arr_pos = piece.position.arr_pos();
        board[arr_pos] = Some(piece);
    }
    board
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x:{}, y:{}}}", self.x, self.y)
    }
}

enum Direction {
    Up,
    UpRight,
    UpLeft,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    // move_copy creates a Position copy moved by given amount of fields in given direction
    fn move_copy(&self, direction: Direction, amount: usize) -> Result<Position, Error> {
        let mut pos = Position::new(self.x, self.y);
        match direction {
            Direction::Up => pos.y += amount,
            Direction::UpRight => {
                pos.x += amount;
                pos.y += amount;
            }
            Direction::UpLeft => {
                if amount > pos.x || (amount + pos.y) > 7 {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.x -= amount;
                pos.y += amount;
            }
        };
        if !pos.is_valid() {
            Err(Error::PositionOutOfBounds)
        } else {
            Ok(pos)
        }
    }

    // arr_pos gives a current position as an index of 1D array
    fn arr_pos(&self) -> usize {
        self.y * 8 + self.x
    }

    // is_valid reports whether the position lays within board bounds
    fn is_valid(&self) -> bool {
        self.x < 8 && self.y < 7
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Piece {
    position: Position,
    starting_position: bool,
}

impl Piece {
    fn new(position: Position, starting_position: bool) -> Piece {
        Piece {
            position,
            starting_position,
        }
    }

    fn possible_moves<'a>(&self, board: &Board) -> Vec<Position> {
        let mut moves = Vec::new();
        let adjacent_pos = match self.position.move_copy(Direction::Up, 1) {
            Ok(pos) => pos,
            Err(_) => return moves,
        };

        if let Some(_) = board.collision(&adjacent_pos) {
            return moves;
        }
        moves.push(adjacent_pos);

        let starting_move_pos = match self.position.move_copy(Direction::Up, 2) {
            Ok(pos) => pos,
            Err(_) => return moves,
        };
        if self.starting_position {
            if let None = board.collision(&starting_move_pos) {
                moves.push(starting_move_pos);
            }
        }
        moves
    }

    fn possible_strikes<'a>(&self, board: &Board) -> Vec<Position> {
        let mut strikes = Vec::new();
        let strike_directions = vec![Direction::UpLeft, Direction::UpRight];
        for direction in strike_directions {
            if let Ok(strike_position) = self.position.move_copy(direction, 1) {
                if let Some(_) = board.enemy_collision(&strike_position) {
                    strikes.push(strike_position);
                }
            }
        }
        strikes
    }

    fn possible_actions<'a>(&self, board: &Board) -> Vec<Position> {
        let mut actions = self.possible_moves(&board);
        actions.extend(self.possible_strikes(&board));
        actions
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

fn main() {
    let p1 = Piece::new(Position { x: 0, y: 1 }, true);
    let p2 = Piece::new(Position { x: 1, y: 1 }, true);
    let p3 = Piece::new(Position { x: 0, y: 6 }, true);
    let mut board = Board {
        my_pieces: vec![p1, p2],
        enemy_pieces: vec![p3],
    };

    print!("starting board:\n{}", board);

    for i in 1..6 {
        let possible_actions = board.possible_actions();
        let action = possible_actions.choose(&mut rand::thread_rng()).unwrap();
        board = board.play(action).unwrap();
        print!("after move #{}:\n{}\n", i, board);
    }
}

#[test]
fn position_to_arr() {
    assert_eq!(20, Position { x: 4, y: 2 }.arr_pos());
    assert_eq!(0, Position { x: 0, y: 0 }.arr_pos());
    assert_eq!(63, Position { x: 7, y: 7 }.arr_pos());
    assert_eq!(9, Position { x: 1, y: 1 }.arr_pos());
    assert_eq!(7, Position { x: 7, y: 0 }.arr_pos());
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

#[test]
fn pawn_possible_strikes() {
    let me1 = Piece::new(Position::new(3, 1), true);
    let me2 = Piece::new(Position::new(7, 1), true);
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
        me1.possible_strikes(&board),
        "pawn {} strikes",
        me1.position
    );
    assert_eq!(
        vec![enemy3.position],
        me2.possible_strikes(&board),
        "pawn {} strikes on the right edge",
        me2.position
    );
    assert_eq!(
        vec![enemy5.position],
        me5.possible_strikes(&board),
        "pawn {} strikes on the left edge",
        me5.position
    );
    let empty_pos: Vec<Position> = Vec::new();
    assert_eq!(
        empty_pos,
        me3.possible_strikes(&board),
        "pawn {} no targets",
        me3.position
    );
    assert_eq!(
        vec![enemy4.position],
        me4.possible_strikes(&board),
        "pawn {} one target",
        me4.position
    );
}

#[test]
fn board_possible_actions() {
    let p1 = Piece::new(Position { x: 0, y: 1 }, true);
    let p2 = Piece::new(Position { x: 3, y: 2 }, false);
    let p3 = Piece::new(Position { x: 1, y: 2 }, false);
    let board = Board {
        my_pieces: vec![p1, p2],
        enemy_pieces: vec![p3],
    };

    let expected = vec![
        PieceAction {
            piece: &p1,
            destination: Position::new(0, 2),
        },
        PieceAction {
            piece: &p1,
            destination: Position::new(0, 3),
        },
        PieceAction {
            piece: &p1,
            destination: Position::new(1, 2),
        },
        PieceAction {
            piece: &p2,
            destination: Position::new(3, 3),
        },
    ];
    assert_eq!(expected, board.possible_actions());
}

#[test]
fn swap_sides() {
    let p1 = Piece::new(Position { x: 0, y: 1 }, true);
    let p2 = Piece::new(Position { x: 1, y: 1 }, true);
    let p3 = Piece::new(Position { x: 0, y: 6 }, true);
    let mut board = Board {
        my_pieces: vec![p1, p2],
        enemy_pieces: vec![p3],
    };
    board.swap_sides();
    assert_eq!(vec![p3], board.my_pieces);
    assert_eq!(vec![p1, p2], board.enemy_pieces);
}
