use std::fmt;
use itertools::Itertools;

use coord::Dim2 as Coord;
use cell::Cell;
use grid::{ Grid, GridItem, GridCoord };


#[derive(Debug, Clone, Eq)]
pub struct Population {
    cells: Vec<Cell>,
    size: (usize, usize),
    gen: usize,
}

impl Population {
    pub fn new(cells: Vec<Cell>, gen: usize) -> Self {
        let size = f32::sqrt(cells.len() as f32) as usize;

        Population {
            cells: cells,
            size: (size, size),
            gen: gen
        }
    }

    pub fn empty(size: usize) -> Self {
        vec![false; size * size].into()
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn evolve(&self) -> Self {
        let mut vec = vec![];
        let size = self.size();

        for coord in iproduct!(0..size, 0..size) {
            vec.push(self.item_fate(coord.into()));
        }

        Population::new(vec, self.gen + 1)
    }



    pub fn regenerate<T: Into<Coord>>(&mut self, coord: T) {
        let coord: Coord = coord.into();
        let (x, y) = coord.into();
        let size = self.size();
        self.cells[x * size + y] = Cell::Alive;
    }

    pub fn generation(&self) -> usize {
        self.gen
    }
}


impl IntoIterator for Population {
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


impl Grid for Population {
    type Item = Cell;
    type Coord = Coord;

    fn size(&self) -> usize {
        self.size.0
    }


    fn item(&self, coord: Coord) -> Cell {
        let (x, y) = coord.into();
        self.cells[x * self.size() + y]
    }


    fn item_neighbours(&self, coord: Coord) -> Vec<Cell> {
        let (x, y) = coord.into();
        let size = self.size();

        [(x, dec(y, size)),
         (x, inc(y, size)),
         (dec(x, size), y),
         (inc(x, size), y),
         (dec(x, size), dec(y, size)),
         (inc(x, size), dec(y, size)),
         (dec(x, size), inc(y, size)),
         (inc(x, size), inc(y, size))]
            .into_iter()
            .map(|&x| self.item(x.into()))
            .collect()
    }
}



impl fmt::Display for Population {
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


impl PartialEq for Population {
    fn eq(&self, other: &Population) -> bool {
        self.cells == other.cells
    }
}

impl From<Vec<bool>> for Population {
    fn from(xs: Vec<bool>) -> Population {
        Population::new(xs.into_iter().map(|x| x.into()).collect(), 1)
    }
}

impl From<Vec<Cell>> for Population {
    fn from(xs: Vec<Cell>) -> Population {
        Population::new(xs, 1)
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


fn dec(x: usize, n: usize) -> usize {
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


pub fn glider(mut population: Population, offset: (usize, usize)) -> Population {
    glider_br(population, offset)
}

/// Glider
///
/// ```ignore
/// _ # _ _ _    _ _ _ _ _    _ _ _ _ _
/// _ _ # _ _    # _ # _ _    _ _ # _ _
/// # # # _ _    _ # # _ _    # _ # _ _
/// _ _ _ _ _    _ # _ _ _    _ # # _ _
/// _ _ _ _ _    _ _ _ _ _    _ _ _ _ _
/// ```
pub fn glider_br(mut population: Population, offset: (usize, usize)) -> Population {
    let (x, y) = offset;

    population.regenerate((x + 0, y + 1));
    population.regenerate((x + 1, y + 2));
    population.regenerate((x + 2, y + 0));
    population.regenerate((x + 2, y + 1));
    population.regenerate((x + 2, y + 2));

    population
}

/// # # # _ _
/// # _ _ _ _
/// _ # _ _ _
/// _ _ _ _ _
/// _ _ _ _ _
pub fn glider_tl(mut population: Population, offset: (usize, usize)) -> Population {
    let (x, y) = offset;

    population.regenerate((x + 0, y + 0));
    population.regenerate((x + 0, y + 1));
    population.regenerate((x + 0, y + 2));
    population.regenerate((x + 1, y + 0));
    population.regenerate((x + 2, y + 1));

    population
}

/// # # # _ _
/// _ _ # _ _
/// _ # _ _ _
/// _ _ _ _ _
/// _ _ _ _ _
pub fn glider_bl(mut population: Population, offset: (usize, usize)) -> Population {
    let (x, y) = offset;

    population.regenerate((x + 0, y + 0));
    population.regenerate((x + 0, y + 1));
    population.regenerate((x + 0, y + 2));
    population.regenerate((x + 1, y + 2));
    population.regenerate((x + 2, y + 1));

    population
}

/// _ # _ _ _
/// # _ _ _ _
/// # # # _ _
/// _ _ _ _ _
/// _ _ _ _ _
pub fn glider_tr(mut population: Population, offset: (usize, usize)) -> Population {
    let (x, y) = offset;

    population.regenerate((x + 0, y + 1));
    population.regenerate((x + 1, y + 0));
    population.regenerate((x + 2, y + 0));
    population.regenerate((x + 2, y + 1));
    population.regenerate((x + 2, y + 2));

    population
}

#[test]
fn test_glider() {
    let size = 5;
    let offset = (1, 1);
    let ppl = vec![false; size * size];
    let glr = glider(ppl.into(), offset);
    let alive = glr.cells().into_iter().filter(|&x| x.is_alive()).count();

    let xs = vec![
        ((1, 2), Cell::Alive),
        ((2, 3), Cell::Alive),
        ((3, 1), Cell::Alive),
        ((3, 2), Cell::Alive),
        ((3, 3), Cell::Alive),
    ];

    for ((x, y), expected) in xs {
        assert_eq!(glr.item(x, y), expected);
    }

    assert_eq!(alive, 5);
}

// blinker =
//     [ (0, 0), (1, 0), (2, 0) ]


// toad =
//     [         (1, 0), (2, 0), (3, 0)
//     , (0, 1), (1, 1), (2, 1)
//     ]

// beacon =
//     [ (0, 0), (1, 0)
//     , (0, 1), (1, 1)
//     ,                 (2, 2), (3, 2)
//     ,                 (2, 3), (3, 3)
//     ]

// acorn =
//     [         (1, 0)
//     ,                         (3, 1)
//     , (0, 2), (1, 2),                (4, 2), (5, 2), (6, 2)
//     ]



#[test]
fn test_fate() {
    let size = 5;
    let offset = (1, 1);
    let ppl = glider(vec![false; size * size].into(), offset);

    let xs = vec![
        ((0, 0), Cell::Unborn),
        ((1, 1), Cell::Unborn),
        ((2, 2), Cell::Unborn),
        ((3, 3), Cell::Alive),
        ((4, 4), Cell::Unborn),
        ((1, 2), Cell::Dead(0)),
    ];

    for ((x, y), expected) in xs {
        assert_eq!(ppl.item_fate(x, y), expected);
    }
}


#[test]
fn test_neigbours() {
    let size = 5;
    let offset = (1, 1);
    let ppl = glider(vec![false; size * size].into(), offset);
    let ns = ppl.item_neighbours(1, 2);

    let expected: Population = vec![
        false, false, false,
        false,        false,
        false, false, true ,
    ].into();

    assert_eq!(ns, *expected.cells());
    assert_eq!(ns.into_iter().filter(|&x| x.is_alive()).count(), 1);
}
