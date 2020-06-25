pub mod board;
pub mod errors;
pub mod pieces;
pub mod position;
pub mod ray;

use position::Position;

#[derive(Eq, PartialEq, std::fmt::Debug)]
pub struct Action {
    source: Position,
    destination: Position,
}

impl Action {
    fn new(source: Position, destination: Position) -> Self {
        Self {
            source,
            destination,
        }
    }
}
