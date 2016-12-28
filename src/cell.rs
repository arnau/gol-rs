use std::fmt;
use std::str::FromStr;


#[derive(Debug, Copy, Clone, Eq)]
pub enum Cell {
    Alive,
    Dead(usize),
    Unborn,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            _ => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        match *self {
            Cell::Dead(_) => true,
            _ => false,
        }
    }

    pub fn is_unborn(&self) -> bool {
        match *self {
            Cell::Unborn => true,
            _ => false,
        }
    }

    pub fn rot(&self) -> Cell {
        match *self {
            Cell::Dead(x) => Cell::Dead(x + 1),
            x => x,
        }
    }

}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.is_alive() == other.is_alive() ||
        self.is_dead() == other.is_dead() ||
        self.is_unborn() == other.is_unborn()
    }
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Alive => write!(f, "#"),
            // Cell::Dead(x) => write!(f, "{}", x),
            Cell::Dead(x) => write!(f, "_"),
            Cell::Unborn => write!(f, "_"),
        }
    }
}


impl From<usize> for Cell {
    fn from(x: usize) -> Cell {
        if x == 1 {
            Cell::Alive
        } else {
            Cell::Unborn
        }
    }
}

impl From<bool> for Cell {
    fn from(x: bool) -> Cell {
        if x {
            Cell::Alive
        } else {
            Cell::Unborn
        }
    }
}


impl FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Cell, String> {
        match s {
            "#" => Ok(Cell::Alive),
            "_" => Ok(Cell::Unborn),
            _ => Err("Unknown state".into()),
        }
    }
}
