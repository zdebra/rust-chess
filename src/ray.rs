use super::pieces::Piece;
use super::position::{Direction, Position};
use super::Action;

#[derive(std::fmt::Debug, PartialEq, Eq)]
pub struct Ray {
    start: Position,
    direction: Direction,
    limit: usize,
}

impl Ray {
    pub fn new(start: Position, direction: Direction, limit: usize) -> Self {
        Self {
            start,
            direction,
            limit,
        }
    }

    pub fn new_from(x: usize, y: usize, direction: Direction, limit: usize) -> Self {
        Self {
            start: Position::new(x, y),
            direction,
            limit,
        }
    }

    /// Yields Vec of valid Actions for piece in starting position of the ray
    pub fn move_actions(&self, my_pieces: &Vec<Piece>, enemy_pieces: &Vec<Piece>) -> Vec<Action> {
        let actions = vec![];
        // for pos in self {
        //     match my_pieces.iter().find(|&piece| piece.get_position() == pos) {}
        // }
        actions
    }
}

impl IntoIterator for Ray {
    type Item = Position;
    type IntoIter = RayIntoIterator;

    /// Hello
    ///
    /// # Examples
    /// ```
    /// use chess::position::{Direction, Position};
    /// use chess::ray::Ray;
    ///
    /// let ray = Ray::new_from(4, 4, Direction::Up, 2);
    ///
    /// let mut positions = ray.into_iter();
    ///
    /// assert_eq!(Some(Position::new(4, 5)), positions.next());
    /// assert_eq!(Some(Position::new(4, 6)), positions.next());
    /// assert_eq!(Some(Position::new(4, 7)), positions.next());
    /// assert_eq!(None, positions.next());
    /// assert_eq!(None, positions.next());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        RayIntoIterator {
            ray: self,
            index: 1,
            disposed: false,
        }
    }
}

impl<'a> IntoIterator for &'a Ray {
    type Item = Position;
    type IntoIter = RayIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        RayIterator {
            ray: self,
            index: 1,
            disposed: false,
        }
    }
}

pub struct RayIntoIterator {
    ray: Ray,
    index: usize,
    disposed: bool,
}

impl Iterator for RayIntoIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.disposed {
            return None;
        }
        match self.ray.start.move_copy(self.ray.direction, self.index) {
            Ok(pos) => {
                self.index += 1;
                Some(pos)
            }
            Err(_) => {
                self.disposed = true;
                return None;
            }
        }
    }
}

pub struct RayIterator<'a> {
    ray: &'a Ray,
    index: usize,
    disposed: bool,
}

impl<'a> Iterator for RayIterator<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.disposed {
            return None;
        }
        match self.ray.start.move_copy(self.ray.direction, self.index) {
            Ok(pos) => {
                self.index += 1;
                Some(pos)
            }
            Err(_) => {
                self.disposed = true;
                return None;
            }
        }
    }
}