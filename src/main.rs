use rand::seq::SliceRandom;
use std::fmt;
mod errors;
mod pieces;
mod position;

pub struct Board {
    my_pieces: Vec<pieces::Piece>,
    enemy_pieces: Vec<pieces::Piece>,
}

impl fmt::Display for Board {
    // https://chess.stackexchange.com/questions/1600/chess-program-for-linux-unix-console
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let my_pieces_space = pieces::to_space(&self.my_pieces);
        let enemy_pieces_space = pieces::to_space(&self.enemy_pieces);

        let board_strs: Vec<String> = my_pieces_space
            .iter()
            .zip(enemy_pieces_space.iter())
            .rev()
            .map(|(my_place, enemy_place)| match (my_place, enemy_place) {
                (Some(_), None) => "♙".to_string(),
                (None, Some(_)) => "♟".to_string(),
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
    fn my_collision(&self, position: &position::Position) -> Option<&pieces::Piece> {
        for piece in self.my_pieces.iter() {
            if &piece.position == position {
                return Some(piece);
            }
        }
        None
    }
    fn enemy_collision(&self, position: &position::Position) -> Option<&pieces::Piece> {
        for piece in self.enemy_pieces.iter() {
            if &piece.position == position {
                return Some(piece);
            }
        }
        None
    }
    fn collision(&self, position: &position::Position) -> Option<&pieces::Piece> {
        if let Some(my_piece) = self.my_collision(position) {
            Some(my_piece)
        } else if let Some(enemy_piece) = self.enemy_collision(position) {
            Some(enemy_piece)
        } else {
            None
        }
    }

    fn possible_actions(&self) -> Vec<pieces::PieceAction> {
        let mut actions = Vec::new();
        for piece in self.my_pieces.iter() {
            for action in piece.possible_actions(&self) {
                actions.push(pieces::PieceAction {
                    piece: piece,
                    destination: action,
                });
            }
        }
        actions
    }

    fn swap_sides(&self) -> Board {
        // std::mem::swap(&mut self.my_pieces, &mut self.enemy_pieces);
        let mut swapped_board = Board {
            enemy_pieces: self.my_pieces.clone(),
            my_pieces: self.enemy_pieces.clone(),
        };

        pieces::swap_positions(&mut swapped_board.enemy_pieces);
        pieces::swap_positions(&mut swapped_board.my_pieces);
        swapped_board
    }

    fn play(&self, action: &pieces::PieceAction) -> Result<Board, errors::Error> {
        if !self.possible_actions().iter().any(|a| a == action) {
            return Err(errors::Error::InvalidAction);
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
        Ok(output_board)
    }
}

fn main() {
    let p1 = pieces::Piece::new(position::Position { x: 0, y: 1 }, true);
    let p2 = pieces::Piece::new(position::Position { x: 1, y: 1 }, true);
    let p3 = pieces::Piece::new(position::Position { x: 0, y: 6 }, true);
    let mut board = Board {
        my_pieces: vec![p1, p2],
        enemy_pieces: vec![p3],
    };

    for i in 1..6 {
        print!("starting board:\n{}\n", board);
        let possible_actions = board.possible_actions();
        // print!("possible actions:{:?}\n", possible_actions);
        let action = possible_actions.choose(&mut rand::thread_rng()).unwrap();
        board = board.play(action).unwrap();
        print!("after move #{}:\n{}\n", i, board);
        board = board.swap_sides()
    }
}

#[test]
fn board_possible_actions() {
    let p1 = pieces::Piece::new(position::Position { x: 0, y: 1 }, true);
    let p2 = pieces::Piece::new(position::Position { x: 3, y: 2 }, false);
    let p3 = pieces::Piece::new(position::Position { x: 1, y: 2 }, false);
    let board = Board {
        my_pieces: vec![p1, p2],
        enemy_pieces: vec![p3],
    };

    let expected = vec![
        pieces::PieceAction {
            piece: &p1,
            destination: position::Position::new(0, 2),
        },
        pieces::PieceAction {
            piece: &p1,
            destination: position::Position::new(0, 3),
        },
        pieces::PieceAction {
            piece: &p1,
            destination: position::Position::new(1, 2),
        },
        pieces::PieceAction {
            piece: &p2,
            destination: position::Position::new(3, 3),
        },
    ];
    assert_eq!(expected, board.possible_actions());
}

#[test]
fn swap_sides() {
    let p1 = pieces::Piece::new(position::Position { x: 0, y: 1 }, true);
    let p2 = pieces::Piece::new(position::Position { x: 1, y: 1 }, true);
    let p3 = pieces::Piece::new(position::Position { x: 0, y: 6 }, true);
    let mut board = Board {
        my_pieces: vec![p1, p2],
        enemy_pieces: vec![p3],
    };
    board = board.swap_sides();
    assert_eq!(
        vec![pieces::Piece::new(position::Position { x: 0, y: 1 }, true)],
        board.my_pieces
    );
    assert_eq!(
        vec![
            pieces::Piece::new(position::Position { x: 0, y: 6 }, true),
            pieces::Piece::new(position::Position { x: 1, y: 6 }, true)
        ],
        board.enemy_pieces
    );
}
