mod dijkstra;
mod dir;
mod map;
mod pos;

use map::Map;
fn main() {
    let map = Map::from_file("example_input");
    map.print();
    let _cost = dijkstra::shortest_path_cost(&map);
}
