use crate::dir::{DIRS, DOWN, RIGHT};
use crate::map::Map;
use crate::pos::Pos;
use ansi_term::Colour;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::f32::consts::E;

#[derive(Debug)]
struct SolutionCell {
    prev_pos: Pos,
    cost: u32,
    have_best_path: bool,
}

struct SolutionMap {
    cells: Vec<Vec<Option<SolutionCell>>>,
}

impl SolutionMap {
    fn new(map: &Map) -> Self {
        let mut cells = Vec::new();
        for _ in 0..map.y_size() {
            let mut row = Vec::new();
            for _ in 0..map.x_size() {
                row.push(None);
            }
            cells.push(row);
        }
        SolutionMap { cells }
    }

    fn x_size(&self) -> usize {
        self.cells[0].len()
    }

    fn y_size(&self) -> usize {
        self.cells.len()
    }

    fn print(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Some(_) => {
                        let cost = cell.as_ref().unwrap().cost;
                        let have_best_path = cell.as_ref().unwrap().have_best_path;
                        if have_best_path {
                            print!("{}", Colour::Green.paint(format!("{:_>3}", cost)));
                        } else {
                            print!("{}", Colour::Yellow.paint(format!("{:_>3}", cost)));
                        }
                    }
                    None => print!("   "),
                }
                let pos = Pos {
                    y: y as i32,
                    x: x as i32,
                };
                if x < self.x_size() - 1 {
                    let c = self.hor_link_char(&pos);
                    print!("{c}")
                }
            }
            println!();
            if y < self.y_size() - 1 {
                for x in 0..self.x_size() {
                    let pos = Pos {
                        y: y as i32,
                        x: x as i32,
                    };
                    let c = self.ver_link_char(&pos);
                    print!(" {c}  ")
                }
                println!();
            }
        }
    }

    fn hor_link_char(&self, pos: &Pos) -> char {
        let cell = &self.cells[pos.y as usize][pos.x as usize].as_ref();
        if let Some(cell) = cell {
            let nbr_pos = pos + &RIGHT;
            let nbr_cell = &self.cells[nbr_pos.y as usize][nbr_pos.x as usize].as_ref();
            if let Some(nbr_cell) = nbr_cell {
                if nbr_cell.prev_pos == *pos {
                    return '←';
                } else if cell.prev_pos == nbr_pos {
                    return '→';
                }
            }
        }
        ' '
    }

    fn ver_link_char(&self, pos: &Pos) -> char {
        let cell = &self.cells[pos.y as usize][pos.x as usize].as_ref();
        if let Some(cell) = cell {
            let nbr_pos = pos + &DOWN;
            let nbr_cell = &self.cells[nbr_pos.y as usize][nbr_pos.x as usize].as_ref();
            if let Some(nbr_cell) = nbr_cell {
                if nbr_cell.prev_pos == *pos {
                    return '↑';
                } else if cell.prev_pos == nbr_pos {
                    return '↓';
                }
            }
        }
        ' '
    }

    fn improve_path_to_cell(&mut self, pos: &Pos, cost: u32, prev_pos: &Pos) {
        self.cells[pos.y as usize][pos.x as usize] = Some(SolutionCell {
            prev_pos: prev_pos.clone(),
            cost: cost,
            have_best_path: false,
        });
    }

    fn mark_path_to_cell_as_best(&mut self, pos: &Pos) {
        self.cells[pos.y as usize][pos.x as usize]
            .as_mut()
            .unwrap()
            .have_best_path = true;
    }

    fn too_many_steps_in_same_direction(&self, visit_pos: &Pos, next_pos: &Pos) -> bool {
        let dir_visit_to_next = visit_pos.dir_to_nbr_pos(next_pos);
        let going_dir = dir_visit_to_next.opposite();
        let max_steps = 3;
        let mut current_pos = visit_pos.clone();
        for _ in 0..max_steps {
            let prev_pos = &self.cells[current_pos.y as usize][current_pos.x as usize]
                .as_ref()
                .unwrap()
                .prev_pos;
            let dir_current_to_prev = current_pos.dir_to_nbr_pos(&prev_pos);
            if dir_current_to_prev != going_dir {
                return false;
            }
            current_pos = prev_pos.clone();
        }
        true
    }
}

pub fn shortest_path_cost(map: &Map) -> usize {
    let mut solution_map = SolutionMap::new(&map);
    let start_pos = Pos { y: 0, x: 0 };
    let goal_pos = Pos {
        y: (map.y_size() - 1) as i32,
        x: (map.x_size() - 1) as i32,
    };
    solution_map.improve_path_to_cell(&start_pos, map.get(&start_pos), &start_pos);
    solution_map.print();
    let mut unvisited_positions = PriorityQueue::new();
    unvisited_positions.push(start_pos.clone(), Reverse(0));
    loop {
        let visit_pos = match unvisited_positions.pop() {
            Some((pos, _)) => pos,
            None => panic!("Not possible to reach goal"),
        };
        if visit_pos == goal_pos {
            println!(
                "Reached goal at cost {}",
                solution_map.cells[goal_pos.y as usize][goal_pos.x as usize]
                    .as_ref()
                    .unwrap()
                    .cost
            );
            break;
        }
        solution_map.mark_path_to_cell_as_best(&visit_pos);
        println!("Visiting position: {:?}", visit_pos);
        let visit_cost = solution_map.cells[visit_pos.y as usize][visit_pos.x as usize]
            .as_ref()
            .unwrap()
            .cost;
        for nbr_pos in reachable_neighbors(&map, &visit_pos).iter() {
            println!("  Considering neighbor: {:?}", nbr_pos);
            if solution_map.too_many_steps_in_same_direction(&visit_pos, nbr_pos) {
                println!("    Too many steps in same direction; skipping");
                continue;
            }
            let map_cell_cost = map.get(nbr_pos);
            let sol_cell = solution_map.cells[nbr_pos.y as usize][nbr_pos.x as usize].as_ref();
            let new_cost: u32 = visit_cost + map.get(nbr_pos);
            match sol_cell {
                Some(sol_cell) => {
                    // We already have a path to this neighbor; is the new path better?
                    let current_cost = sol_cell.cost;
                    println!("    Current cost to here: {}", current_cost);
                    // let new_cost = current_cost + map.cost_at(nbr) as u32;
                    // let nbr_cell_opt = &solution_map.cells[nbr.y as usize][nbr.x as usize];
                }
                None => {
                    // We don't yet have a path to this neighbor; add this path.
                    solution_map.cells[nbr_pos.y as usize][nbr_pos.x as usize] =
                        Some(SolutionCell {
                            prev_pos: visit_pos.clone(),
                            cost: new_cost,
                            have_best_path: false,
                        });
                    unvisited_positions.push(nbr_pos.clone(), Reverse(map_cell_cost));
                }
            }
        }
        solution_map.print();
    }
    0
}

fn reachable_neighbors(map: &Map, pos: &Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    for dir in DIRS.iter() {
        let new_pos = pos + dir;
        if map.pos_on_map(&new_pos) {
            neighbors.push(new_pos);
        }
    }
    neighbors
}
