#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate sdl2;
extern crate rand;
#[macro_use] extern crate itertools;
#[macro_use] extern crate ndarray;


pub mod grid;
pub mod coord;
pub mod cell;
pub mod world;
pub mod population;
pub mod community;

// mod board;
pub mod sdl;
