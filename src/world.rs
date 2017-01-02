use grid::Grid;


/// The world of Conway.
#[derive(Debug, Clone)]
pub struct World<T> {
    grid: T,
    size: (usize, usize),
}

impl<T: Grid + Into<T>> World<T> {
    pub fn new(grid: T) -> Self {

        let size = grid.size();

        World {
            grid: grid,
            size: (size, size),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn grid(&self) -> &T {
        &(self.grid)
    }
}


impl<T: Grid + Into<T>> Iterator for World<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.grid.clone();
        let new = self.grid.evolve();

        // if old == new { return None; }

        self.grid = new;

        Some(old)
    }
}
