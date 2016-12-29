extern crate sdl2;
extern crate rand;
#[macro_use] extern crate itertools;
#[macro_use] extern crate ndarray;


mod grid;
mod cell;
mod population;
mod world;

// mod board;
pub mod sdl;


    // let mut a = parse(INPUT);
    // let mut scratch = Board::zeros((N as Ix, N as Ix));
    // let steps = 100;
    // turn_on_corners(&mut a);
    // for _ in 0..steps {
    //     iterate(&mut a, &mut scratch);
    //     turn_on_corners(&mut a);
    //     //render(&a);
    // }
    // render(&a);
    // let alive = a.iter().filter(|&&x| x > 0).count();
    // println!("After {} steps there are {} cells alive", steps, alive);
