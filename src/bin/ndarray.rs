extern crate conway;
#[macro_use] extern crate ndarray;

use ndarray::prelude::*;

use conway::world::World;
use conway::community::{ Community, Glider, Blinker };
use conway::cell::Cell;
use conway::sdl;


fn main() {
    let settings = sdl::Settings {
        delay: 120,
        cell_size: 5,
    };

    let n = 50;
    let patterns = vec![
        // Glider::BottomRight(0, 0),
        // Glider::BottomLeft(25, 25),
        // Glider::TopLeft(14, 19),
        // Glider::TopRight(30, 15),
        Blinker::TopBottom(10, 10),
        Blinker::LeftRight(20, 10),
    ];
    let grid = Community::from((patterns, n));

    // println!("{:?}", grid);
    let world = World::new(grid);
    sdl::run(world, settings);
}
