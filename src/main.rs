use rand::seq::SliceRandom;
mod board;
mod errors;
mod pieces;
mod position;

fn main() {
    let p1 = pieces::Pawn::new(position::Position { x: 0, y: 1 }, true);
    let p2 = pieces::Pawn::new(position::Position { x: 1, y: 1 }, true);
    let p3 = pieces::Pawn::new(position::Position { x: 0, y: 6 }, true);
    let mut board = board::Board {
        my_pieces: vec![Box::new(p1), Box::new(p2)],
        enemy_pieces: vec![Box::new(p3)],
    };

    for i in 1..6 {
        print!("starting board:\n{}\n", board);
        let possible_actions = board.possible_actions();
        // print!("possible actions:{:?}\n", possible_actions);
        let action = possible_actions.choose(&mut rand::thread_rng()).unwrap();
        board.play(action).unwrap();
        print!("after move #{}:\n{}\n", i, board);
        board.swap_sides()
    }
}
