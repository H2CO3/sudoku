use std::ops::{ Index, IndexMut };
use std::fmt::{ Display, Formatter, Result };

#[derive(Debug, Clone)]
struct Grid {
    len: usize,
    cells: Vec<u8>,
}

impl Grid {
    fn size(&self) -> usize {
        self.len * self.len
    }

    fn solve(self) -> Vec<Grid> {
        self.empty_cells().into_iter().fold(vec![self], |grids, index| {
            grids.into_iter().flat_map(|grid| grid.placements(index)).collect::<Vec<_>>()
        })
    }

    fn available_numbers(&self, row: usize, col: usize) -> Vec<u8> {
        debug_assert!(self[(row, col)] == 0);

        let size = self.size();

        let mut numbers: Vec<_> = (1..size as u8 + 1).collect();

        for row in 0..size {
            if let Some(index) = numbers.iter().position(|&n| n == self[(row, col)]) {
                numbers.swap_remove(index);
            }
        }

        for col in 0..size {
            if let Some(index) = numbers.iter().position(|&n| n == self[(row, col)]) {
                numbers.swap_remove(index);
            }
        }

        let box_row = row / self.len;
        let box_col = col / self.len;

        for row in box_row * self.len..(box_row + 1) * self.len {
            for col in box_col * self.len..(box_col + 1) * self.len {
                if let Some(index) = numbers.iter().position(|&n| n == self[(row, col)]) {
                    numbers.swap_remove(index);
                }
            }
        }

        numbers
    }

    fn with_cell_at(&self, cell: u8, row: usize, col: usize) -> Grid {
        debug_assert!(self[(row, col)] == 0);

        let mut grid = self.clone();

        grid[(row, col)] = cell;
        grid
    }

    fn placements(&self, (row, col): (usize, usize)) -> Vec<Grid> {
        self.available_numbers(row, col)
            .into_iter()
            .map(|cell| self.with_cell_at(cell, row, col))
            .collect()
    }

    fn empty_cells(&self) -> Vec<(usize, usize)> {
        let size = self.size();

        (0..size).flat_map(|row| {
            (0..size).map(|col| (row, col))
                .filter(|&coords| self[coords] == 0)
                .collect::<Vec<_>>()
        }).collect()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.cells[row * self.size() + col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let size = self.size();
        &mut self.cells[row * size + col]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let size = self.size();

        for (index, cell) in self.cells.iter().enumerate() {
            if index % size == 0 {
                write!(f, "\n")?;
            }

            write!(f, "{:3}", cell)?;
        }

        Ok(())
    }
}

fn main() {
    let s2 = Grid {
        len: 2,
        cells: vec![
            0, 0, 0, 0,
            0, 0, 2, 1,
            3, 0, 0, 4,
            0, 0, 0, 0,
        ],
    };

    let s3 = Grid {
        len: 3,
        cells: vec![
            0, 0, 1, 6, 9, 0, 5, 0, 0,
            4, 0, 0, 2, 7, 0, 0, 0, 1,
            0, 7, 0, 0, 0, 0, 0, 9, 0,
            0, 0, 0, 0, 0, 0, 0, 3, 0,
            0, 0, 0, 4, 3, 0, 0, 0, 7,
            0, 0, 0, 7, 8, 0, 6, 0, 0,
            0, 0, 6, 0, 0, 0, 8, 0, 5,
            0, 2, 0, 1, 4, 0, 0, 6, 0,
            0, 1, 0, 3, 5, 0, 0, 4, 0,
        ],
    };

    for solution in s3.solve() {
        println!("{}", solution);
    }
}
