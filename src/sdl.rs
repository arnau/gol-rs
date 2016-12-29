use std::fmt;
use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Renderer;
use std::{thread, time};

use grid::Grid;
use coord::Dim2 as Coord;
use population::{ glider, glider_br, glider_bl, glider_tl, glider_tr };
use world::{ World, Population, Cell };


pub fn main() {
    let speed = 10;
    let n = 100;
    let cell_size = 5;
    let (mut r, mut e) = init((n * cell_size) as u32);
    // let mut world = World::glider(n);
    // let mut world = World::random(n);
    // let mut world = World::infinite(n);

    // let ppl = glider(glider(Population::empty(n), (0, 0)), (20, 10));
    let ppl = glider_br(
        glider_bl(
            Population::empty(n), (0, 7)
        ), (28, 0)
    );
    let mut world = World::new(ppl);

    let mut running = false;

    'running:loop {
        for event in e.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    running = !running;
                }
                _ =>  {}
            }
        }

        if running {
            match *(&world.next()) {
                Some(ref grid) => render(&mut r, cell_size, &grid),
                None => {}
            }

            thread::sleep(time::Duration::from_millis(speed));
        }
    }
}


fn init<'a>(size: u32)-> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of Life", size , size)
                                .position_centered()
                                .opengl()
                                .build()
                                .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}


fn render(r: &mut Renderer, cell_size: usize, grid: &Population) {
    let size = grid.size();

    println!("{}\n\n", grid);

    r.set_draw_color(Color::RGB(250, 250, 250));
    r.clear();

    for (coord, cell) in grid.clone().into_iter() {
        display_cell(r, cell_size, coord, cell)
    }

    r.present();
}

fn display_cell(r: &mut Renderer, cell_size: usize, coord: Coord, cell: Cell) {
    let (x, y) = coord.into();

    let mut x = cell_size * x;
    let mut y = cell_size * y;

    let cell_color = match cell {
        Cell::Alive => Color::RGB(0, 255, 0),
        Cell::Unborn => Color::RGB(255, 255, 255),
        Cell::Dead(x) => match x {
            x if x <= 20 => {
                let y = 250 - (x * 10) as u8;
                Color::RGB(y, y, y)
            }
            _ => Color::RGB(0, 0, 0),
        }
    };

    r.set_draw_color(cell_color);
    r.fill_rect(Rect::new(x as i32, y as i32, cell_size as u32, cell_size as u32));
}
