use rand::{self, Rng};

pub use cell::Cell;
pub use population::{ Population, glider };


pub fn sketch() {
    let n = 5;
    let world = World::glider(n);
    // let world = World::random(n);
    // let world = World::new(vec![None; n]);

    for ppl in world {
        println!("{}", ppl);
    }
}



/// The world of Conway.
#[derive(Debug, Clone)]
pub struct World {
    grid: Population,
}

impl World {
    pub fn new(population: Population) -> Self {
        World {
            grid: population,
        }
    }

    pub fn glider(size: usize) -> Self {
        let vec = Population::empty(size);
        let offset = (1, 1);

        World {
            grid: glider(vec, offset),
        }
    }

    pub fn random(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<bool> = Vec::new();

        for _ in  0..(size * size) {
            vec.push(rng.gen());
        }

        World {
            grid: vec.into(),
        }
    }

    pub fn infinite(size: usize) -> Self {
        let mut vec = Population::empty(size);

        vec.regenerate((1, 1));
        vec.regenerate((1, 2));
        vec.regenerate((1, 3));
        vec.regenerate((1, 5));

        vec.regenerate((2, 1));

        vec.regenerate((3, 4));
        vec.regenerate((3, 5));

        vec.regenerate((4, 2));
        vec.regenerate((4, 3));
        vec.regenerate((4, 5));

        vec.regenerate((5, 1));
        vec.regenerate((5, 3));
        vec.regenerate((5, 5));

        World {
            grid: vec,
        }
    }

    pub fn grid(&self) -> &Population {
        &(self.grid)
    }
}


impl Iterator for World {
    type Item = Population;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.grid.clone();
        let new = self.grid.evolve();

        // if old == new { return None; }

        self.grid = new;

        Some(old)
    }
}


#[test]
fn test_fst_gen() {
    let n = 5;
    let mut world = World::glider(n);
    let ppl = world.next().unwrap();
    let cells = ppl.cells().clone();
    let expected: Vec<Cell> = vec![
        false, false, false, false, false,
        false, false, true , false, false,
        false, false, false, true , false,
        false, true , true , true , false,
        false, false, false, false, false,
    ].into_iter().map(|x| x.into()).collect();

    assert_eq!(cells, expected);
}


#[test]
fn test_snd_gen() {
    let n = 5;
    let mut world = World::glider(n);
    world.next();
    let ppl = world.next().unwrap();
    let cells = ppl.cells().clone();

    let expected: Vec<Cell> = vec![
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 0, 1, 0,
        0, 0, 1, 1, 0,
        0, 0, 1, 0, 0,
    ].into_iter().map(|x| x.into()).collect();

    assert_eq!(cells, expected);
}
