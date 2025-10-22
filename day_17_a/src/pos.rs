use crate::dir::{DOWN, Dir, LEFT, RIGHT, UP};
use std::ops;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    // Use signed integers to allow negative off-map positions if needed
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn dir_to_nbr_pos(&self, nbr_pos: &Pos) -> Dir {
        // Assumes nbr_pos is a direct neighbor of self
        if nbr_pos.x > self.x {
            RIGHT
        } else if nbr_pos.x < self.x {
            LEFT
        } else if nbr_pos.y > self.y {
            DOWN
        } else {
            UP
        }
    }
}

impl ops::Add<&Dir> for &Pos {
    type Output = Pos;

    fn add(self, dir: &Dir) -> Pos {
        Pos {
            x: self.x + dir.x_dir,
            y: self.y + dir.y_dir,
        }
    }
}
