extern crate conway;
#[macro_use] extern crate ndarray;

use ndarray::prelude::*;

use conway::world::World;
use conway::community::{ Community, glider_br };
use conway::cell::Cell;
use conway::sdl;


fn main() {
    let settings = sdl::Settings {
        delay: 10,
        cell_size: 5,
    };

    let n = 50;
    let vec: Vec<Cell> = vec![Cell::Unborn; n * n];
    let arr = Array::from_shape_vec((n, n), vec).unwrap();
    let grid  = glider_br(Community::new(arr, n), (0, 7));

    // Idea:
    // let ppl = Population::with([GliderBR(0, 7), GliderBL(28, 0)]);

    let world = World::new(grid);

    sdl::run(world, settings);
}
