//! Grid implementation using `ndarray::Array2<Cell>`.

use itertools::Itertools;
use ndarray::prelude::*;
use rand::{self, Rng};
use std::fmt;

use coord::Dim2 as Coord;
use cell::Cell;
use grid::Grid;
use pattern::*;

type Matrix = Array2<Cell>;

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

    pub fn empty(n: usize) -> Self {
        Community::new(Array2::from_elem((n as Ix, n as Ix), Cell::Unborn), 1)
    }

    /// Mix in a pattern
    pub fn insert<T: Pattern<Array2<Cell>>>(&mut self, layout: Layout<T>) {
        let (x, y) = layout.offset();
        let (n, m) = layout.size();

        if layout.size() > self.size {
            panic!("Patterns must be smaller than the recipient grid");
        }

        if (n + x, m + y) > self.size {
            panic!("Pattern size {:?} with offset {:?} overflows grid of {:?}",
                   layout.size(), layout.offset(), self.size);
        }

        let lower_x = (0 + x) as isize;
        let upper_x = (n + x) as isize;
        let lower_y = (0 + y) as isize;
        let upper_y = (m + y) as isize;

        let a = layout.pattern();

        self.cells
            .slice_mut(s![lower_x..upper_x, lower_y..upper_y])
            .assign(&a);
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


///////////////////////////////////////////////////////////////////////////////
// Patterns
///////////////////////////////////////////////////////////////////////////////


impl Pattern<Matrix> for Blinker {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn pattern(&self) -> Matrix {
        let mut canvas = arr2(&[
            [Cell::Unborn, Cell::Alive, Cell::Unborn],
            [Cell::Unborn, Cell::Alive, Cell::Unborn],
            [Cell::Unborn, Cell::Alive, Cell::Unborn],
        ]);

        match *self {
            Blinker::TopBottom => canvas,
            Blinker::LeftRight => {
                &canvas.invert_axis(Axis(1));
                canvas
            }
        }
    }
}


impl Pattern<Matrix> for Toad {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn pattern(&self) -> Matrix {
        arr2(&[
            [Cell::Unborn, Cell::Unborn, Cell::Unborn, Cell::Unborn],
            [Cell::Unborn, Cell::Alive , Cell::Alive , Cell::Alive],
            [Cell::Alive , Cell::Alive , Cell::Alive , Cell::Unborn],
            [Cell::Unborn, Cell::Unborn, Cell::Unborn, Cell::Unborn],
        ])
    }
}


impl Pattern<Matrix> for Glider {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn pattern(&self) -> Matrix {
        let mut canvas = arr2(&[
            [Cell::Unborn, Cell::Alive , Cell::Unborn],
            [Cell::Unborn, Cell::Unborn, Cell::Alive ],
            [Cell::Alive , Cell::Alive , Cell::Alive ],
        ]);

        match *self {
            Glider::BottomLeft => {
                &canvas.invert_axis(Axis(1));
                canvas
            }
            Glider::BottomRight => {
                canvas
            }
            Glider::TopLeft => {
                &canvas.invert_axis(Axis(0));
                &canvas.invert_axis(Axis(1));
                canvas
            }
            Glider::TopRight => {
                &canvas.invert_axis(Axis(1));
                canvas.reversed_axes()
            }
        }
    }
}


impl Pattern<Matrix> for Beacon {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn pattern(&self) -> Matrix {
        arr2(&[
            [Cell::Alive , Cell::Alive , Cell::Unborn, Cell::Unborn],
            [Cell::Alive , Cell::Unborn, Cell::Unborn, Cell::Unborn],
            [Cell::Unborn, Cell::Unborn, Cell::Unborn, Cell::Alive ],
            [Cell::Unborn, Cell::Unborn, Cell::Alive , Cell::Alive ],
        ])
    }
}


impl Pattern<Matrix> for Pulsar {
    fn size(&self) -> (usize, usize) {
        (15, 15)
    }

    fn pattern(&self) -> Matrix {
        let (n, m) = self.size();
        let raw: Vec<Cell> = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ].into_iter().map(|&x| x.into()).collect();

        Array::from_shape_vec((n as Ix, m as Ix), raw).unwrap()
    }
}

impl Pattern<Matrix> for Pentadecathlon {
    fn size(&self) -> (usize, usize) {
        (16, 9)
    }

    fn pattern(&self) -> Matrix {
        let (n, m) = self.size();
        let raw: Vec<Cell> = [
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ].into_iter().map(|&x| x.into()).collect();

        Array::from_shape_vec((n as Ix, m as Ix), raw).unwrap()
    }
}
    // pub fn random(size: usize) -> Self
    //     where T: From<Vec<bool>> {
    //     let mut rng = rand::thread_rng();
    //     let mut vec: Vec<bool> = Vec::new();

    //     for _ in  0..(size * size) {
    //         vec.push(rng.gen());
    //     }

    //     World {
    //         grid: vec.into(),
    //         size: (size, size),
    //     }
    // }
