//! Grid implementation using `ndarray::Array2<Cell>`.

use std::fmt;
use itertools::Itertools;
use ndarray::prelude::*;

use coord::Dim2 as Coord;
use cell::Cell;
use grid::Grid;


#[derive(Debug, Clone, Eq)]
pub struct Community {
    cells: Array2<Cell>,
    size: (usize, usize),
    gen: usize,
}


impl Community {
    pub fn new(cells: Array2<Cell>, gen: usize) -> Self {
        let size = ((&cells).rows(), (&cells).cols());

        Community {
            cells: cells,
            size: size,
            gen: gen
        }
    }
}


impl Grid for Community {
    type Cell = Cell;
    type Coord = Coord;

    fn size(&self) -> usize {
        self.size.0
    }


    fn item(&self, coord: Coord) -> Cell {
        let (x, y) = coord.into();
        self.cells[[x, y]]
    }


    fn item_neighbours(&self, coord: Coord) -> Vec<Cell> {
        let (x, y) = coord.into();
        let size = self.size();

        vec![
            self.cells[[x, dec(y, size)]],
            self.cells[[x, inc(y, size)]],
            self.cells[[dec(x, size), y]],
            self.cells[[inc(x, size), y]],
            self.cells[[dec(x, size), dec(y, size)]],
            self.cells[[dec(x, size), inc(y, size)]],
            self.cells[[inc(x, size), dec(y, size)]],
            self.cells[[inc(x, size), inc(y, size)]],
        ]
    }

    fn evolve(&self) -> Self {
        let mut vec: Vec<Cell> = vec![];
        let width = self.cells.rows();
        let height = self.cells.cols();

        for (x, y) in iproduct!(0..width, 0..height) {
            vec.push(self.item_fate((x, y).into()));
        }

        let arr = Array::from_shape_vec((width, height), vec).unwrap();

        Community::new(arr, self.gen + 1)
    }
}


impl IntoIterator for Community {
    type Item = (Coord, Cell);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
            .enumerate()
            .map(|(i, &x)| (Coord::from_index(i, self.size()), x))
            .collect::<Vec<(Coord, Cell)>>()
            .into_iter()
    }
}


impl PartialEq for Community {
    fn eq(&self, other: &Community) -> bool {
        self.cells == other.cells
    }
}


impl fmt::Display for Community {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = self.cells.clone().into_iter()
            .map(|x| format!("{}", x))
            .chunks(self.size())
            .into_iter()
            .map(|xs| format!("{}\n", xs.collect::<String>()))
            .collect::<String>();

        write!(f, "{}", res)
    }
}



fn inc(x: usize, n: usize) -> usize {
    let x = x + 1;

    if x >= n {
        x % n
    } else {
        x
    }
}


fn dec(x: usize, n: usize) -> usize {
    if x == 0 {
        n - 1
    } else {
        x - 1
    }
}


pub fn glider_br(mut grid: Community, offset: (usize, usize)) -> Community {
    let (x, y) = offset;

    grid.cells[[x    , y + 1]] = Cell::Alive;
    grid.cells[[x + 1, y + 2]] = Cell::Alive;
    grid.cells[[x + 2, y    ]] = Cell::Alive;
    grid.cells[[x + 2, y + 1]] = Cell::Alive;
    grid.cells[[x + 2, y + 2]] = Cell::Alive;

    grid
}
