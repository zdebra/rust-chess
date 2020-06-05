use rand::seq::SliceRandom;
mod board;
mod errors;
mod pieces;
mod position;

fn main() {
    let mut board = board::Board {
        my_pieces: vec![
            Box::new(pieces::Pawn::new(position::Position { x: 0, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 1, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 2, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 3, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 4, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 5, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 6, y: 1 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 7, y: 1 }, true)),
            Box::new(pieces::Rook::new(position::Position { x: 0, y: 0 })),
            Box::new(pieces::Rook::new(position::Position { x: 7, y: 0 })),
            Box::new(pieces::Bishop::new(position::Position { x: 2, y: 0 })),
            Box::new(pieces::Bishop::new(position::Position { x: 5, y: 0 })),
            Box::new(pieces::Queen::new(position::Position { x: 3, y: 0 })),
        ],
        enemy_pieces: vec![
            Box::new(pieces::Pawn::new(position::Position { x: 0, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 1, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 2, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 3, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 4, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 5, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 6, y: 6 }, true)),
            Box::new(pieces::Pawn::new(position::Position { x: 7, y: 6 }, true)),
            Box::new(pieces::Rook::new(position::Position { x: 0, y: 7 })),
            Box::new(pieces::Rook::new(position::Position { x: 7, y: 7 })),
            Box::new(pieces::Bishop::new(position::Position { x: 2, y: 7 })),
            Box::new(pieces::Bishop::new(position::Position { x: 5, y: 7 })),
            Box::new(pieces::Queen::new(position::Position { x: 3, y: 7 })),
        ],
    };

    for i in 1..6 {
        print!("starting board:\n{}\n", board);
        let possible_actions = board.possible_actions();
        let action = possible_actions.choose(&mut rand::thread_rng()).unwrap();
        board.play(action).unwrap();
        print!("after move #{}:\n{}\n", i, board);
        board.swap_sides()
    }
}
