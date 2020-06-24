pub mod errors;
pub mod pieces;
pub mod position;
pub mod ray;

use position::Position;

pub struct Action {
    pub source: Position,
    pub destination: Position,
}
