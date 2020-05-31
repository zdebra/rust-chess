#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("position out of bounds")]
    PositionOutOfBounds,
    #[error("action is invalid")]
    InvalidAction,
}
