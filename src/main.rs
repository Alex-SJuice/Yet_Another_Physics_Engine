extern crate nalgebra as na;

mod simulation;
mod objects;

use objects::{PointMass, Wall};
use simulation::{rendering::{ObjectDraw}};
use na::{Vector2};
use raylib::prelude::*;

fn main() {
    let (mut handles, win, mut tm) = simulation::initialize()
        .title("Simulation")
        .window_size(500, 500)
        .window_scale(50.0, 50.0)
        .window_position(-25.0, 50.0)
        .build();
    let mut accumulator = 0.0;

    let mut a = PointMass::new(Vector2::new(0.0, 50.0), 1.0, 1.0);
    let w = Wall::new(Vector2::new(0.0, 10.0), Vector2::new(0.0, 1.0));

    a.apply_force(Vector2::new(0.0, -9.8));
    while !handles.window_should_close() {
        tm.update();
        accumulator += tm.delta().as_secs_f32();
        if accumulator >= 1.0 / 60.0 {
            
            //println!("{:?}", a.velocity());
            //a.collide_wall(&w);
            a.collide_floor(10.0);
            a.update_position(accumulator);
            accumulator = 0.0;
            let mut draw = handles.begin_drawing();
            draw.clear_background(Color::BLACK);
            draw.draw_pointmass(&win, &a);
        }
    }
}
