use super::errors::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x:{}, y:{}}}", self.x, self.y)
    }
}

#[derive(Copy, Clone, std::fmt::Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
    // move_copy creates a Position copy moved by given amount of fields in given direction
    pub fn move_copy(&self, direction: Direction, amount: usize) -> Result<Position, Error> {
        let mut pos = Position::new(self.x, self.y);
        match direction {
            Direction::Up => {
                if (pos.y + amount) > 7 {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.y += amount
            }
            Direction::Down => {
                if amount > pos.y {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.y -= amount;
            }
            Direction::Left => {
                if amount > pos.x {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.x -= amount;
            }
            Direction::Right => {
                if (amount + pos.x) > 7 {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.x += amount;
            }
            Direction::UpRight => {
                if (pos.y + amount) > 7 || (pos.x + amount) > 7 {
                    return Err(Error::PositionOutOfBounds);
                }
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
            Direction::DownRight => {
                if (amount + pos.x) > 7 || amount > pos.y {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.x += amount;
                pos.y -= amount;
            }
            Direction::DownLeft => {
                if amount > pos.x || amount > pos.y {
                    return Err(Error::PositionOutOfBounds);
                }
                pos.x -= amount;
                pos.y -= amount;
            }
        };
        if !pos.is_valid() {
            Err(Error::PositionOutOfBounds)
        } else {
            Ok(pos)
        }
    }

    // arr_pos gives a current position as an index of 1D array
    pub fn arr_pos(&self) -> usize {
        self.y * 8 + self.x
    }

    // is_valid reports whether the position lays within board bounds
    fn is_valid(&self) -> bool {
        self.x < 8 && self.y < 8
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
