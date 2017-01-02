//! Grid traits. Abstraction over a Conway grid, a collection of items with a
//! set of rules to decide their next state (fate).
//!
//! Conway rules:
//!
//! 1. Any live cell with fewer than two live neighbours dies, as if caused
//! by underpopulation.
//! 2. Any live cell with two or three live neighbours lives on to the next
//! generation.
//! 3. Any live cell with more than three live neighbours dies, as if by
//! overpopulation.
//! 4. Any dead cell with exactly three live neighbours becomes a live cell,
//! as if by reproduction.

use std::fmt::Display;


pub trait Grid: Clone + Display + IntoIterator {
    type Cell: GridItem;
    type Coord: GridCoord + Clone;


    /// The size of the grid. E.g. A 3 x 3 grid has size 3.
    fn size(&self) -> usize;

    /// The value of the item positioned at the given coords.
    fn item(&self, coord: Self::Coord) -> Self::Cell;

    /// The item neighbours.
    fn item_neighbours(&self, coord: Self::Coord) -> Vec<Self::Cell>;

    fn evolve(&self) -> Self;

    /// Evolves the item to its next state.
    fn item_fate(&self, coord: Self::Coord) -> Self::Cell {
        let count = self.item_neighbours(coord.clone())
                        .into_iter()
                        .filter(|x| x.is_alive())
                        .count();
        let cell = self.item(coord);
        let is_alive = (&cell).is_alive();

        match (is_alive,  count) {
            (true, 2...3) => cell.keep(),
            (true, _)     => cell.kill(),
            (false, 3)    => cell.revive(),
            (false, _)    => cell.rot(),
        }
    }
}


pub trait GridItem {
    fn is_alive(&self) -> bool;

    /// Keep as is.
    fn keep(&self) -> Self;

    /// Kill if alive.
    fn kill(&self) -> Self;
    
    /// Rot if dead.
    fn rot(&self) -> Self;

    /// Revive if not alive.
    fn revive(&self) -> Self;
}

pub trait GridCoord {}



///////////////////////////////////////////////////////////////////////////////
// Infinite grid
///////////////////////////////////////////////////////////////////////////////

pub fn inc(x: usize, n: usize) -> usize {
    let x = x + 1;

    if x >= n {
        x % n
    } else {
        x
    }
}

#[test]
fn test_inc() {
    let size = 3;
    let xs = vec![
        (0, 1),
        (1, 2),
        (2, 0),
    ];

    for (x, y) in xs {
        assert_eq!(inc(x, size), y)
    }
}


pub fn dec(x: usize, n: usize) -> usize {
    if x == 0 {
        n - 1
    } else {
        x - 1
    }
}

#[test]
fn test_dec() {
    let size = 3;
    let xs = vec![
        (0, 2),
        (1, 0),
        (2, 1),
    ];

    for (x, y) in xs {
        assert_eq!(dec(x, size), y)
    }
}
