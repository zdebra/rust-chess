// use super::errors::Error;
// use super::position::Position;
// use std::fmt;

// /// Represents a board from perspective of the player that is about to play, hence
// /// my_pieces and enemy_pieces fields.
// pub struct Board {
//     pub my_pieces: Vec<Box<dyn Piece>>,
//     pub enemy_pieces: Vec<Box<dyn Piece>>,
// }

// impl Board {
//     /// Returns Some with a reference to my own colliding piece on a given position.
//     /// Returns None if there is no colliding my piece.
//     pub fn my_collision(&self, position: Position) -> Option<&Box<dyn Piece>> {
//         for piece in self.my_pieces.iter() {
//             if piece.get_position() == position {
//                 return Some(piece);
//             }
//         }
//         None
//     }

//     /// Returns Some with a reference to enemy colliding piece on a given position.
//     /// Returns None if there is no colliding enemy piece.
//     pub fn enemy_collision(&self, position: Position) -> Option<&Box<dyn Piece>> {
//         for piece in self.enemy_pieces.iter() {
//             if piece.get_position() == position {
//                 return Some(piece);
//             }
//         }
//         None
//     }
//     /// Returns Some with a reference to a colliding piece on a given [position].
//     /// Returns None if there is no colliding piece.
//     pub fn collision(&self, position: Position) -> Option<&Box<dyn Piece>> {
//         if let Some(my_piece) = self.my_collision(position) {
//             Some(my_piece)
//         } else if let Some(enemy_piece) = self.enemy_collision(position) {
//             Some(enemy_piece)
//         } else {
//             None
//         }
//     }

//     /// Returns Vec of all possible actions in a current turn
//     pub fn possible_actions(&self) -> Vec<Action> {
//         let mut actions = Vec::new();
//         for piece in self.my_pieces.iter() {
//             for action in piece.possible_actions(&self) {
//                 actions.push(Action {
//                     source: piece.get_position(),
//                     destination: action,
//                 });
//             }
//         }
//         actions
//     }

//     /// Mutates internal state to swap the sides. This is usually used after each turn.
//     pub fn swap_sides(&mut self) {
//         std::mem::swap(&mut self.my_pieces, &mut self.enemy_pieces);
//         swap_positions(&mut self.enemy_pieces);
//         swap_positions(&mut self.my_pieces);
//     }

//     /// Executes given action. Yields error if action is not valid.
//     pub fn play(&mut self, action: &Action) -> Result<(), Error> {
//         if !self.possible_actions().iter().any(|a| a == action) {
//             return Err(Error::InvalidAction);
//         }
//         // remove enemy piece if the action destination collides with it
//         if let Some(enemy_index) = self
//             .enemy_pieces
//             .iter()
//             .position(|piece| piece.get_position() == action.destination)
//         {
//             self.enemy_pieces.remove(enemy_index);
//         }

//         // set position of the piece to the destination
//         self.my_pieces
//             .iter_mut()
//             .find(|piece| piece.get_position() == action.source)
//             .unwrap()
//             .set_position(action.destination);
//         Ok(())
//     }

//     pub fn check_position(&self, pos: Position) -> bool {
//         // self.swap_sides();
//         let is_empty = self
//             .enemy_pieces
//             .iter()
//             .map(|enemy_piece| {
//                 enemy_piece
//                     .allowed_strike_destinations(self)
//                     .into_iter()
//                     .find(|&p| pos == p)
//             })
//             .flatten()
//             .collect::<Vec<Position>>()
//             .is_empty();
//         // self.swap_sides();
//         is_empty
//     }
// }

// impl fmt::Display for Board {
//     // https://chess.stackexchange.com/questions/1600/chess-program-for-linux-unix-console
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let my_pieces_space = to_space(&self.my_pieces);
//         let enemy_pieces_space = to_space(&self.enemy_pieces);

//         let board_strs: Vec<String> = my_pieces_space
//             .iter()
//             .zip(enemy_pieces_space.iter())
//             .rev()
//             .map(|(my_place, enemy_place)| match (my_place, enemy_place) {
//                 (Some(piece), None) => piece.icon().light.to_string(),
//                 (None, Some(piece)) => piece.icon().dark.to_string(),
//                 _ => ".".to_string(),
//             })
//             .collect();

//         f.write_str("  a b c d e f g h\n")?;
//         for (row_index, row) in board_strs.chunks(8).enumerate() {
//             f.write_fmt(format_args!(
//                 "{} {} {} {} {} {} {} {} {} {}\n",
//                 8 - row_index,
//                 row[7],
//                 row[6],
//                 row[5],
//                 row[4],
//                 row[3],
//                 row[2],
//                 row[1],
//                 row[0],
//                 8 - row_index,
//             ))?;
//         }
//         f.write_str("  a b c d e f g h\n")?;
//         Ok(())
//     }
// }

// #[test]
// fn board_possible_actions() {
//     use super::pieces::Pawn;

//     let p1_pos = Position { x: 0, y: 1 };
//     let p2_pos = Position { x: 3, y: 2 };
//     let p3_pos = Position { x: 1, y: 2 };
//     let p1 = Pawn::new(p1_pos, true);
//     let p2 = Pawn::new(p2_pos, false);
//     let p3 = Pawn::new(p3_pos, false);
//     let board = Board {
//         my_pieces: vec![Box::new(p1), Box::new(p2)],
//         enemy_pieces: vec![Box::new(p3)],
//     };

//     let expected = vec![
//         Action {
//             source: p1_pos,
//             destination: Position::new(0, 2),
//         },
//         Action {
//             source: p1_pos,
//             destination: Position::new(0, 3),
//         },
//         Action {
//             source: p1_pos,
//             destination: Position::new(1, 2),
//         },
//         Action {
//             source: p2_pos,
//             destination: Position::new(3, 3),
//         },
//     ];
//     assert_eq!(expected, board.possible_actions());
// }

// #[test]
// fn swap_sides() {
//     use super::pieces::Pawn;
//     let p1 = Pawn::new(Position { x: 0, y: 1 }, true);
//     let p2 = Pawn::new(Position { x: 1, y: 1 }, true);
//     let p3 = Pawn::new(Position { x: 0, y: 6 }, true);
//     let mut board = Board {
//         my_pieces: vec![Box::new(p1), Box::new(p2)],
//         enemy_pieces: vec![Box::new(p3)],
//     };
//     board.swap_sides();
//     let expected: Vec<Box<dyn Piece>> = vec![Box::new(Pawn::new(Position { x: 0, y: 1 }, true))];
//     assert_eq!(expected, board.my_pieces);
//     let expected: Vec<Box<dyn Piece>> = vec![
//         Box::new(Pawn::new(Position { x: 0, y: 6 }, true)),
//         Box::new(Pawn::new(Position { x: 1, y: 6 }, true)),
//     ];
//     assert_eq!(expected, board.enemy_pieces);
// }

// // #[test]
// // fn check_position_simple() {
// //     use super::pieces::Pawn;
// //     let p1 = Pawn::new(Position { x: 0, y: 1 }, true);
// //     let p2 = Pawn::new(Position { x: 1, y: 1 }, true);
// //     let p3 = Pawn::new(Position { x: 0, y: 6 }, true);
// //     let mut board = Board {
// //         my_pieces: vec![Box::new(p1), Box::new(p2)],
// //         enemy_pieces: vec![Box::new(p3)],
// //     };

// //     assert_eq!(true, board.check_position(Position { x: 1, y: 5 }));
// //     assert_ne!(true, board.check_position(Position { x: 0, y: 6 }));
// // }
