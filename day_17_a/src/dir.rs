#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dir {
    pub x_dir: i32,
    pub y_dir: i32,
}

pub const UP: Dir = Dir {
    x_dir: 0,
    y_dir: -1,
};

pub const DOWN: Dir = Dir { x_dir: 0, y_dir: 1 };

pub const LEFT: Dir = Dir {
    x_dir: -1,
    y_dir: 0,
};

pub const RIGHT: Dir = Dir { x_dir: 1, y_dir: 0 };

pub const DIRS: [Dir; 4] = [UP, DOWN, LEFT, RIGHT];

impl Dir {
    pub fn opposite(&self) -> Dir {
        Dir {
            x_dir: -self.x_dir,
            y_dir: -self.y_dir,
        }
    }
}
