use cell::Cell;

pub trait Grid {
    fn cell(&self, x: usize, y: usize) -> Cell;
}
