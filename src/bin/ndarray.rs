extern crate conway;
#[macro_use] extern crate ndarray;

use ndarray::prelude::*;

use conway::world::World;
use conway::community::{ Community, glider_br, Glider };
use conway::cell::Cell;
use conway::sdl;


fn main() {
    let settings = sdl::Settings {
        delay: 10,
        cell_size: 5,
    };

    let n = 50;
    let patterns = vec![
        Glider::BottomRight(0, 0),
        Glider::BottomLeft(25, 25),
        Glider::TopLeft(14, 19),
        Glider::TopRight(30, 15),
    ];
    let grid = Community::from((patterns, n));

    // println!("{:?}", grid);
    let world = World::new(grid);
    sdl::run(world, settings);


    // let a = arr2(&[
    //     [0, 1, 0],
    //     [0, 0, 1],
    //     [1, 1, 1],
    // ]);

    // let mut g = Array2::from_elem((5 as Ix, 5 as Ix), Cell::Unborn);
    // let a = arr2(&[
    //     [Cell::Unborn, Cell::Alive , Cell::Unborn],
    //     [Cell::Unborn, Cell::Unborn, Cell::Alive ],
    //     [Cell::Alive , Cell::Alive , Cell::Alive ],
    // ]);
    // g.slice_mut(s![1..-1, 1..-1]).assign(&a);

    // let i = arr2(&[
    //     [1, 0, 0],
    //     [0, 1, 0],
    //     [0, 0, 1],
    // ]);
    // let i = Array::eye(3);

    // let mut c = a.reversed_axes();
    // println!("\n\n{:?}", &c);

    // c.invert_axis(Axis(1));
    // println!("\n\n{:?}", &c);


    // println!("\n\n{:?}", &g);
}
