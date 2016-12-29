use grid::GridCoord;


#[derive(Debug, Copy, Clone)]
pub struct Dim2(pub usize, pub usize);

impl Dim2 {
    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn from_index(index: usize, size: usize) -> Self {
        From::from((index % size, index / size))
    }
}

impl GridCoord for Dim2 {}

impl From<(usize, usize)> for Dim2 {
    fn from(x: (usize, usize)) -> Self {
        Dim2(x.0, x.1)
    }
}

impl<'a> From<&'a (usize, usize)> for Dim2 {
    fn from(x: &(usize, usize)) -> Self {
        Dim2(x.0, x.1)
    }
}

impl From<Dim2> for (usize, usize) {
    fn from(x: Dim2) -> (usize, usize) {
        (x.0, x.1)
    }
}

impl<'a> From<&'a Dim2> for (usize, usize) {
    fn from(x: &Dim2) -> (usize, usize) {
        (x.0, x.1)
    }
}
