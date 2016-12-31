extern crate conway;

use conway::world::World;
use conway::population::Population;
use conway::sdl;
use conway::population::{ glider_br, glider_bl };

fn main() {
    let settings = sdl::Settings {
        delay: 10,
        cell_size: 5,
    };

    let n = 100;
    // let mut world = World::glider(n);
    // let mut world = World::random(n);
    // let mut world = World::infinite(n);

    let ppl = glider_br(
        glider_bl(
            Population::empty(n), (0, 7)
        ), (28, 0)
    );

    // Idea:
    // let ppl = Population::with([GliderBR(0, 7), GliderBL(28, 0)]);

    let world = World::new(ppl);


    sdl::run(world, settings);
}
