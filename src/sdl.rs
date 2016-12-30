use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2;
use std::fmt::Display;
use std::{ thread, time };

use coord::Dim2 as Coord;
use grid::Grid;
use world::{ World, Cell };


#[derive(Debug)]
pub struct Settings {
    pub delay: usize,
    pub cell_size: usize,
}


pub fn run(mut world: World, settings: Settings) {
    let delay = settings.delay;
    let cell_size = settings.cell_size;
    let (width, _) = world.size();

    let (mut r, mut e) = init((width * cell_size) as u32);

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
            if let Some(grid) = world.next() {
                render(&mut r, cell_size, grid);
            }

            thread::sleep(time::Duration::from_millis(delay as u64));
        }
    }
}


fn render<G>(r: &mut Renderer, cell_size: usize, grid: G)
    where G: Grid + Display + IntoIterator<Item = (Coord, Cell)> {

    println!("{}\n\n", grid);

    r.set_draw_color(Color::RGB(250, 250, 250));
    r.clear();

    for (coord, cell) in grid {
        render_cell(r, cell_size, coord, cell)
    }

    r.present();
}

#[allow(unused_must_use)]
fn render_cell(r: &mut Renderer, cell_size: usize, coord: Coord, cell: Cell) {
    let x = cell_size * coord.x();
    let y = cell_size * coord.y();

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
