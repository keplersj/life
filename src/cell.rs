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

        if self == Cell::Alive {
            if alive_cells < 2 {
                return Cell::Dead;
            }

            if alive_cells == 2 || alive_cells == 3 {
                return Cell::Alive;
            }

            if alive_cells > 3 {
                return Cell::Dead;
            }
        }

        if self == Cell::Dead {
            if alive_cells == 3 {
                return Cell::Alive;
            }

            return Cell::Dead;
        }

        self
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
