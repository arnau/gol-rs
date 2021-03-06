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


// Custom


#[derive(Debug, Clone)]
pub struct Random(pub usize, pub usize);


// Still lifes


#[derive(Debug, Clone)]
pub struct Block;

#[derive(Debug, Clone)]
pub struct Beehive;

#[derive(Debug, Clone)]
pub struct Loaf;

#[derive(Debug, Clone)]
pub struct Boat;

#[derive(Debug, Clone)]
pub struct Tub;


// Oscillators


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


// Spaceships


#[derive(Debug, Clone)]
pub enum LightweightSpaceship {
    Right,
    Left,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub enum Glider {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}
