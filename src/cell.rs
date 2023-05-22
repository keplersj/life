use std::fmt::{Display, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}

type Neighbors = [[Option<Cell>; 3]; 3];

impl Cell {
    pub fn tick(self, neighbors: Neighbors) -> Cell {
        let alive_cells = neighbors
            .into_iter()
            .flatten()
            .flatten()
            .filter(Cell::is_alive)
            .count();

        match self {
            Cell::Alive => {
                if (2..=3).contains(&alive_cells) {
                    return Cell::Alive;
                }

                Cell::Dead
            }
            Cell::Dead => {
                if alive_cells == 3 {
                    return Cell::Alive;
                }

                Cell::Dead
            }
        }
    }

    pub fn alive(self) -> Option<Cell> {
        match self {
            Cell::Alive => Some(self),
            _ => None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive().is_some()
    }

    pub fn dead(self) -> Option<Cell> {
        match self {
            Cell::Dead => Some(self),
            _ => None,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead().is_some()
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::Alive => '█',
            Cell::Dead => '░',
        };

        f.write_char(char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_cell_dead_neighbors() {
        let cell = Cell::Dead;
        let neighbors = [
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
            [Some(Cell::Dead), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(cell, next);
        assert_eq!(next, Cell::Dead);
    }

    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.

    #[test]
    fn alive_cell_dead_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
            [Some(Cell::Dead), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    #[test]
    fn alive_cell_one_live_neighbor() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Dead), Some(Cell::Dead)],
            [Some(Cell::Dead), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    // Any live cell with two or three live neighbours lives on to the next generation.

    #[test]
    fn alive_cell_two_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Dead)],
            [Some(Cell::Dead), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Alive);
    }

    #[test]
    fn alive_cell_three_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Dead), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Alive);
    }

    // Any live cell with more than three live neighbours dies, as if by overpopulation.

    #[test]
    fn alive_cell_four_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Alive), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    #[test]
    fn alive_cell_five_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Alive), None, Some(Cell::Alive)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    #[test]
    fn alive_cell_six_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Alive), None, Some(Cell::Alive)],
            [Some(Cell::Alive), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    #[test]
    fn alive_cell_seven_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Alive), None, Some(Cell::Alive)],
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    #[test]
    fn alive_cell_all_live_neighbors() {
        let cell = Cell::Alive;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Alive), None, Some(Cell::Alive)],
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Dead);
    }

    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

    #[test]
    fn dead_cell_three_live_neighbors() {
        let cell = Cell::Dead;
        let neighbors = [
            [Some(Cell::Alive), Some(Cell::Alive), Some(Cell::Alive)],
            [Some(Cell::Dead), None, Some(Cell::Dead)],
            [Some(Cell::Dead), Some(Cell::Dead), Some(Cell::Dead)],
        ];

        let next = cell.tick(neighbors);

        assert_eq!(next, Cell::Alive);
    }

    #[test]
    fn dead_cell_is_dead() {
        let cell = Cell::Dead;
        let is_dead = cell.is_dead();

        assert!(is_dead);
    }

    #[test]
    fn dead_cell_is_not_alive() {
        let cell = Cell::Dead;
        let is_alive = cell.is_alive();

        assert!(!is_alive);
    }

    #[test]
    fn alive_cell_is_alive() {
        let cell = Cell::Alive;
        let is_alive = cell.is_alive();

        assert!(is_alive);
    }

    #[test]
    fn alive_cell_is_not_dead() {
        let cell = Cell::Alive;
        let is_dead = cell.is_dead();

        assert!(!is_dead);
    }
}
