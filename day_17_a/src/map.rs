use crate::pos::Pos;
use std::fs::read_to_string;

pub struct Map {
    cells: Vec<Vec<u32>>,
}

impl Map {
    pub fn from_file(file_path: &str) -> Self {
        let mut cells = Vec::new();
        for line in read_to_string(file_path).unwrap().lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let num = c.to_digit(10).unwrap();
                row.push(num);
            }
            cells.push(row);
        }
        for row in &cells {
            assert!(
                row.len() == cells[0].len(),
                "Inconsistent row lengths in map file"
            );
        }
        Map { cells }
    }

    pub fn print(&self) {
        for row in &self.cells {
            for &cell in row {
                print!("{cell}");
            }
            println!();
        }
    }

    pub fn x_size(&self) -> usize {
        self.cells[0].len()
    }

    pub fn y_size(&self) -> usize {
        self.cells.len()
    }

    pub fn pos_on_map(&self, pos: &Pos) -> bool {
        pos.y >= 0 && pos.y < self.y_size() as i32 && pos.x >= 0 && pos.x < self.x_size() as i32
    }

    pub fn get(&self, pos: &Pos) -> u32 {
        assert!(self.pos_on_map(pos));
        self.cells[pos.y as usize][pos.x as usize]
    }
}
