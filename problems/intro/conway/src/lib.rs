#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    rows_m1: usize,
    cols: usize,
    cols_m1: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        Self {
            rows: rows,
            rows_m1: rows - 1,
            cols: cols,
            cols_m1: cols - 1,
            grid: Vec::<T>::new(),
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        Self {
            rows: rows,
            rows_m1: rows - 1,
            cols: cols,
            cols_m1: cols - 1,
            grid: grid.into(), // check this
        }
    }

    pub fn size(&self) -> (usize, usize) {
        // TODO: your code goes here.
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        // TODO: your code goes here.
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        // TODO: your code goes here.
        self.grid[row * self.cols + col] = value;
    }

    pub fn is_current_cell(&self, x: i32, row: i32, col: i32) -> bool {
        x == row * (self.cols as i32) + col
    }

    pub fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        // TODO: your code goes here.
        let col_as_int = col as i32;
        let col_as_int_p1 = col as i32 + 1;
        let col_as_int_m1 = col as i32 - 1;

        let row_as_int = row as i32;
        let row_as_int_p1 = row as i32 + 1;
        let row_as_int_m1 = row as i32 - 1;
        let lower = (row_as_int_m1 * self.cols as i32 + col_as_int_m1
            ..=row_as_int_m1 * self.cols as i32 + col_as_int_p1)
            .filter(move |x| !x.is_negative() && x / self.cols as i32 == row_as_int_m1)
            .map(|x| {
                ( 
                    (x / self.cols as i32) as usize,
                    (x % self.cols as i32) as usize,
                )
            });

        let curr = (row_as_int * self.cols as i32 + col_as_int_m1
            ..=(row_as_int) * self.cols as i32 + col_as_int_p1)
            .filter(move |x| {
                !x.is_negative()
                    && !self.is_current_cell(*x, row_as_int, col_as_int)
                    && x / self.cols as i32 == row_as_int
            })
            .map(|x| {
                (
                    (x / self.cols as i32) as usize,
                    (x % self.cols as i32) as usize,
                )
            });

        let higher = (row_as_int_p1 * self.cols as i32 + col_as_int_m1
            ..=row_as_int_p1 * self.cols as i32 + col_as_int_p1)
            .filter(move |x| {
                row_as_int_p1 < self.rows as i32
                    && !x.is_negative()
                    && x / self.cols as i32 == row_as_int_p1
            })
            .map(|x| {
                (
                    (x / self.cols as i32) as usize,
                    (x % self.cols as i32) as usize,
                )
            });

        let v = lower.chain(curr).chain(higher);
        v
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self {
            grid: Grid::<Cell>::from_slice(&grid.grid, grid.size().0, grid.size().1),
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        // TODO: your code goes here.
        &self.grid
    }

    pub fn step(&mut self) {
        // TODO: your code goes here.
        let curr_grid = self.get_grid();
        let (rows, cols) = curr_grid.size();
        // Can do better
        let new_grid_vec: Vec<Cell> = curr_grid.grid.iter().copied().collect();
        let mut new_grid: Grid<Cell> = Grid::from_slice(&new_grid_vec[0..], rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                let n_alive: usize = curr_grid
                    .neighbours(r, c)
                    // .iter() // was used with Vec collection iface
                    .filter(|(y, x)| *curr_grid.get(*y, *x) == Cell::Alive)
                    .count();
                let cell_state: &Cell = curr_grid.get(r, c);
                match (cell_state, n_alive) {
                    (Cell::Alive, 2..=3) => new_grid.set(Cell::Alive, r, c),
                    (Cell::Dead, 3) => new_grid.set(Cell::Alive, r, c),
                    _ => new_grid.set(Cell::Dead, r, c),
                }
            }
        }
        *self = Self::from_grid(new_grid);
    }
}
