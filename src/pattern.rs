use ndarray::prelude::*;

use cell::Cell;


type Matrix = Array2<Cell>;
type Offset = (usize, usize);


pub trait Pattern<T> {
    fn size(&self) -> (usize, usize);
    fn pattern(&self) -> T;
}


#[derive(Debug, Clone)]
pub struct Layout<P> {
    pattern: P,
    offset: Offset,
}

impl<T: Pattern<Matrix>> Layout<T> {
    pub fn new(offset: Offset, pattern: T) -> Self {
        Layout {
            pattern: pattern,
            offset: offset,
        }
    }

    pub fn offset(&self) -> Offset {
        self.offset
    }

    pub fn size(&self) -> (usize, usize) {
        self.pattern.size()
    }

    pub fn pattern(&self) -> Matrix {
        self.pattern.pattern()
    }
}


#[derive(Debug, Clone)]
pub struct Random(pub usize, pub usize);

#[derive(Debug, Clone)]
pub struct Blinker;

#[derive(Debug, Clone)]
pub struct Toad;

#[derive(Debug, Clone)]
pub struct Beacon;

#[derive(Debug, Clone)]
pub struct Pulsar;

#[derive(Debug, Clone)]
pub struct Pentadecathlon;


#[derive(Debug, Clone)]
pub enum Glider {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}
