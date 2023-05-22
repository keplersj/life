#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
use life::{cell::Cell, grid::Grid};

fn main() {
    let grid = Grid::new(vec![
        vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
        vec![Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
        vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
        vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive, Cell::Alive],
        vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive, Cell::Alive],
    ]);

    for grid in grid {
        println!("{grid}");
        println!("=====");
    }
}
