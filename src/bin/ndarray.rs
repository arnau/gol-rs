extern crate conway;
#[macro_use] extern crate ndarray;

use ndarray::prelude::*;

use conway::world::World;
use conway::pattern::*;
use conway::community::Community;
use conway::cell::Cell;
use conway::sdl;


fn main() {
    let settings = sdl::Settings {
        delay: 150,
        cell_size: 10,
    };

    let n = 50;
    let mut grid = Community::empty(n);
    // let mut grid = Community::random(n);

    // grid.insert(Layout::new((5, 5), Toad));
    // grid.insert(Layout::new((10, 15), Blinker::TopBottom));
    // grid.insert(Layout::new((25, 20), Glider::BottomRight));
    // grid.insert(Layout::new((0, 0), Glider::TopRight));
    // grid.insert(Layout::new((16, 16), Glider::BottomRight));
    // grid.insert(Layout::new((45, 35), Beacon));
    // grid.insert(Layout::new((20, 20), Pulsar));
    // grid.insert(Layout::new((20, 20), Pentadecathlon));
    grid.insert(Layout::new((20, 20), LightweightSpaceship::Top));


    // println!("{:?}", grid);
    let world = World::new(grid);
    sdl::run(world, settings);
}
