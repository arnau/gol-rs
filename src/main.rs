extern crate sdl2;
extern crate rand;

use rand::Rng;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Renderer;
use std::{thread, time};


const MAX_X: u32 = 199;
const MAX_Y: u32 = MAX_X;
const CELL_WIDTH: u32 = 5;
const CELL_HEIGHT: u32 = CELL_WIDTH;
const NCELLS: u32 = (MAX_X + 1) / CELL_WIDTH;


fn life_random(ncells: u32) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();

    let mut v:Vec<Vec<bool>> = Vec::new();

    for i in  0..ncells {
        v.push(Vec::new());
        for j in  0..ncells {
            v[i as usize].push(rng.gen());
        }
    }

    v
}

// An alternative initial state
// You can use this instead of life_random

fn glider(ncells: u32) -> Vec<Vec<bool>> {
    let mut v:Vec<Vec<bool>> = Vec::new();

    for i in 0..ncells {
        v.push(Vec::new());
        for j in 0..ncells {
            v[i as usize].push(false);
        }
    }

    v[10][11] = true;
    v[11][12] = true;
    v[12][10] = true;
    v[12][11] = true;
    v[12][12] = true;

    v
}

// Another alternative initial state
// You can use this instead of life_random

fn infinite1(ncells: u32) -> Vec<Vec<bool>> {
    let mut v:Vec<Vec<bool>> = Vec::new();

    for i in 0..ncells {
        v.push(Vec::new());
        for j in 0..ncells {
            v[i as usize].push(false);
        }
    }

    v[11][11] = true;
    v[11][12] = true;
    v[11][13] = true;
    v[11][15] = true;

    v[12][11] = true;

    v[13][14] = true;
    v[13][15] = true;

    v[14][12] = true;
    v[14][13] = true;
    v[14][15] = true;

    v[15][11] = true;
    v[15][13] = true;
    v[15][15] = true;

    v
}

fn display_cell(r: &mut Renderer, row: u32, col: u32, n: u32) {

    let mut x = CELL_WIDTH * col;
    let mut y = CELL_WIDTH * row;

    let cell_color = match n {
        0 => Color::RGB(0, 0, 0),
        1 => Color::RGB(75, 75, 75),
        2 => Color::RGB(100, 100, 100),
        3 => Color::RGB(150, 150, 150),
        4 => Color::RGB(200, 200, 200),
        _ => Color::RGB(255, 0, 0),
    };

    r.set_draw_color(cell_color);
    r.fill_rect(Rect::new(x as i32, y as i32, CELL_WIDTH, CELL_HEIGHT));

}

fn display_frame(r: &mut Renderer, v: &Vec<Vec<bool>>) {
    r.set_draw_color(Color::RGB(250, 250, 250));
    r.clear();
    for i in 0..NCELLS {
        for j in 0..NCELLS {
            if v[i as usize][j as usize] {
                let n = count_surrounding(i, j, &v);

                display_cell(r, i, j, n);
            }
        }
    }
    r.present();
}

fn inc(n: usize) ->  usize {
    (n + 1) % (NCELLS as usize)
}

fn dec(n: usize) -> usize {
    if n == 0 {
        (NCELLS - 1) as usize
    } else {
        (n - 1) as usize
    }
}

fn count_surrounding(r: u32, c: u32,
                     v: &Vec<Vec<bool>>) -> u32{
    let r = r as usize;
    let c = c as usize;

    v[dec(r)][c] as u32 +
    v[inc(r)][c] as u32 +
    v[r][dec(c)] as u32 +
    v[r][inc(c)] as u32 +
    v[dec(r)][dec(c)] as u32 +
    v[dec(r)][inc(c)] as u32 +
    v[inc(r)][inc(c)] as u32 +
    v[inc(r)][dec(c)] as u32
}


fn alive(r: u32, c: u32,
         v: &Vec<Vec<bool>>) -> bool {

    let n = count_surrounding(r, c, v);
    let curr = v[r as usize][c as usize] as u32;

    match (curr,  n) {
        (1, 0...1) => false,
        (1, 4...8) => false,
        (1, 2...3) => true,
        (0, 3)     => true,
        (0, 0...2) => false,
        (0, 4...8) => false,
        _ => panic!("alive: error in match"),
    }
}

fn life_next(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut v2:Vec<Vec<bool>> = Vec::new();

    for i in 0..NCELLS {
            v2.push(Vec::new());
        for j in 0..NCELLS {
            if alive(i,  j, &v) {
                v2[i as usize].push(true);
            } else {
                v2[i as usize].push(false);
            }
        }
    }

    v2
}

fn  init<'a>()-> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", MAX_X as u32 + 1 , MAX_Y as u32 + 1)
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

fn main() {
    let (mut r,mut e) = init();
    // let mut v = life_random(NCELLS);
    // let mut v = glider(NCELLS);
    let mut v = infinite1(NCELLS);


    'running:loop {
        for event in e.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => { break 'running },
                _ =>  {}
            }
        }
        display_frame(&mut r, &v);
        v = life_next(v);
        thread::sleep(time::Duration::from_millis(100));
    }
}
