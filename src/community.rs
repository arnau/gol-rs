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

    pub fn empty(n: usize) -> Self {
        Community::new(Array2::from_elem((n as Ix, n as Ix), Cell::Unborn), 1)
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
    let size = grid.size();
    let lower_x = 0 + x as isize;
    let upper_x = 3 + x as isize;
    let lower_y = 0 + y as isize;
    let upper_y = 3 + y as isize;

    let a = arr2(&[
        [Cell::Unborn, Cell::Alive , Cell::Unborn],
        [Cell::Unborn, Cell::Unborn, Cell::Alive ],
        [Cell::Alive , Cell::Alive , Cell::Alive ],
    ]);
    let b = arr2(&[
        [Cell::Unborn, Cell::Alive , Cell::Alive],
        [Cell::Alive , Cell::Unborn, Cell::Alive ],
        [Cell::Unborn, Cell::Unborn, Cell::Alive ],
    ]);




    grid.cells.slice_mut(s![lower_x..upper_x, lower_y..upper_y]).assign(&a);

    grid
}


pub trait Pattern {
    fn size(&self) -> (usize, usize);
    fn offset(&self) -> (usize, usize);
    fn pattern(&self) -> Array2<Cell>;
}


#[derive(Debug, Clone)]
pub enum Glider {
    BottomLeft(usize, usize),
    BottomRight(usize, usize),
    TopLeft(usize, usize),
    TopRight(usize, usize),
}

impl Pattern for Glider {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn offset(&self) -> (usize, usize) {
        match *self {
            Glider::BottomLeft(x, y) => (x, y),
            Glider::BottomRight(x, y) => (x, y),
            Glider::TopLeft(x, y) => (x, y),
            Glider::TopRight(x, y) => (x, y),
        }
    }

    fn pattern(&self) -> Array2<Cell> {
        let (n, m) = self.size();
        let mut canvas = Array2::from_elem((n as Ix, m as Ix), Cell::Unborn);
        let mut base = arr2(&[
            [Cell::Unborn, Cell::Alive , Cell::Unborn],
            [Cell::Unborn, Cell::Unborn, Cell::Alive ],
            [Cell::Alive , Cell::Alive , Cell::Alive ],
        ]);

        let layout = match *self {
            Glider::BottomLeft(_, _) => {
                &base.invert_axis(Axis(1));
                base
            }
            Glider::BottomRight(_, _) => {
                base
            }
            Glider::TopLeft(_, _) => {
                &base.invert_axis(Axis(0));
                &base.invert_axis(Axis(1));
                base
            }
            Glider::TopRight(_, _) => {
                &base.invert_axis(Axis(1));
                base.reversed_axes()
            }
        };

        // canvas.slice_mut(s![1..-1, 1..-1]).assign(&layout);
        canvas.assign(&layout);

        canvas
    }
}

impl<T: Pattern> From<(Vec<T>, usize)> for Community {
    fn from(input: (Vec<T>, usize)) -> Community {
        let (patterns, n) = input;
        let mut grid = Community::empty(n);

        for pattern in patterns {
            if pattern.size() <= (n, n) {
                assign(&mut grid, pattern);
            } else {
                panic!("Patterns must be smaller than the recipient grid");
            }
        }

        grid
    }
}


fn assign<T: Pattern>(mut grid: &mut Community, pattern: T) {
    let (x, y) = pattern.offset();
    let (n, m) = pattern.size();

    let lower_x = (0 + x) as isize;
    let upper_x = (n + x) as isize;
    let lower_y = (0 + y) as isize;
    let upper_y = (m + y) as isize;

    let a = pattern.pattern();

    grid.cells
        .slice_mut(s![lower_x..upper_x, lower_y..upper_y])
        .assign(&a);
}
