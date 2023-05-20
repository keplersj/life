use life::cell::Cell;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(Vec<Vec<Cell>>);

impl Grid {
    pub fn tick(self) -> Self {
        Grid(
            self.0
                .clone()
                .into_iter()
                .enumerate()
                .map(|(row_in, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(|(col_in, cell)| {
                            cell.tick([
                                [
                                    row_in
                                        .checked_sub(1)
                                        .and_then(|idx| self.0.get(idx))
                                        .and_then(|cols| {
                                            col_in
                                                .checked_sub(1)
                                                .and_then(|idx| cols.get(idx))
                                                .copied()
                                        }),
                                    row_in
                                        .checked_sub(1)
                                        .and_then(|idx| self.0.get(idx))
                                        .and_then(|cols| cols.get(col_in).copied()),
                                    row_in
                                        .checked_sub(1)
                                        .and_then(|idx| self.0.get(idx))
                                        .and_then(|cols| {
                                            col_in
                                                .checked_add(1)
                                                .and_then(|idx| cols.get(idx))
                                                .copied()
                                        }),
                                ],
                                [
                                    self.0.get(row_in).and_then(|cols| {
                                        col_in.checked_sub(1).and_then(|idx| cols.get(idx)).copied()
                                    }),
                                    None,
                                    self.0.get(row_in).and_then(|cols| {
                                        col_in.checked_add(1).and_then(|idx| cols.get(idx)).copied()
                                    }),
                                ],
                                [
                                    row_in
                                        .checked_add(1)
                                        .and_then(|idx| self.0.get(idx))
                                        .and_then(|cols| {
                                            col_in
                                                .checked_sub(1)
                                                .and_then(|idx| cols.get(idx))
                                                .copied()
                                        }),
                                    row_in
                                        .checked_add(1)
                                        .and_then(|idx| self.0.get(idx))
                                        .and_then(|cols| cols.get(col_in).copied()),
                                    row_in
                                        .checked_add(1)
                                        .and_then(|idx| self.0.get(idx))
                                        .and_then(|cols| {
                                            col_in
                                                .checked_add(1)
                                                .and_then(|idx| cols.get(idx))
                                                .copied()
                                        }),
                                ],
                            ])
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

struct GridIter(Grid);

impl Iterator for GridIter {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let og = self.0.clone();
        self.0 = self.0.clone().tick();

        og.0.clone()
            .into_iter()
            .flatten()
            .any(|cell| cell.is_alive())
            .then_some(og)
    }
}

impl IntoIterator for Grid {
    type Item = Grid;

    type IntoIter = GridIter;

    fn into_iter(self) -> Self::IntoIter {
        GridIter(self)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .clone()
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|cell| cell.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        )
    }
}

fn main() {
    let grid = Grid(vec![
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
