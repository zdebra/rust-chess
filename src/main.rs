use std::fmt;

struct Board<'a> {
    my_pieces: Vec<&'a Piece>,
    enemy_pieces: Vec<&'a Piece>,
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let my_pieces_space = to_space(&self.my_pieces);
        let board_strs: Vec<String> = my_pieces_space
            .iter()
            .rev()
            .map(|place| match place {
                Some(piece) => "♙".to_string(),
                None => ".".to_string(),
            })
            .collect();

        for (row_index, row) in board_strs.chunks(8).enumerate() {
            f.write_fmt(format_args!(
                "{}. {} {} {} {} {} {} {} {}\n",
                8 - row_index,
                row[7],
                row[6],
                row[5],
                row[4],
                row[3],
                row[2],
                row[1],
                row[0],
            ))?;
        }
        f.write_str("   a b c d e f g h\n")?;
        Ok(())
    }
}

impl<'a> Board<'a> {
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
}

fn to_space<'a>(pieces: &'a Vec<&Piece>) -> [Option<&'a Piece>; 64] {
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

impl Position {
    fn arr_pos(&self) -> usize {
        self.y * 8 + self.x
    }
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    position: Position,
    starting_position: bool,
}

impl Piece {
    fn new(position: Position) -> Piece {
        Piece {
            position,
            starting_position: true,
        }
    }

    fn possible_moves<'a>(&self, board: &Board) -> Vec<Position> {
        let pos = Position {
            x: self.position.x,
            y: self.position.y + 1,
        };
        if let Some(_) = board.my_collision(&pos) {
            vec![pos]
        } else {
            vec![]
        }
    }
}

fn main() {
    let p1 = Piece::new(Position { x: 0, y: 0 });
    let p2 = Piece::new(Position { x: 1, y: 0 });
    let p3 = Piece::new(Position { x: 3, y: 4 });
    let board = Board {
        my_pieces: vec![&p1, &p2],
        enemy_pieces: vec![&p3],
    };

    let possible_moves = p1.possible_moves(&board);
    print!("le board:\n{}", board)
}

#[test]
fn position_to_arr() {
    assert_eq!(20, Position { x: 4, y: 2 }.arr_pos());
    assert_eq!(0, Position { x: 0, y: 0 }.arr_pos());
    assert_eq!(63, Position { x: 7, y: 7 }.arr_pos());
    assert_eq!(9, Position { x: 1, y: 1 }.arr_pos());
    assert_eq!(7, Position { x: 7, y: 0 }.arr_pos());
}
