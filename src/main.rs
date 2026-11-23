extern crate nalgebra as na;

mod simulation;
mod objects;

use objects::PointMass;
use simulation::{TimeManager, rendering::ObjectDraw};
use na::{Vector2};
use raylib::prelude::*;
use rand::prelude::*;
use rand::prelude::*;

fn main() {
    let (mut handles, mut sim) = simulation::initialize()
        .title("Simulation")
        .window_size(500, 500)
        .window_scale(100.0, 100.0)
        .window_position(-50.0, 100.0)
        .build();
    sim.set_constraint(Vector2::<f32>::new(0.0, 50.0), 45.0);

    for i in 0..=15 {
        sim.add_object(PointMass::new(
            Vector2::new(i as f32 * 2.0, 50.0), 
            1.0,
            1.0,
            i == 0));
    }
    for i in 1..=15 {
        sim.add_link(i - 1, i, 2.0);
    }

    let mut render_timer = TimeManager::initialize();
    let mut accumulator = 0.0;

    while !handles.window_should_close() {
        render_timer.update();
        accumulator += render_timer.delta().as_secs_f32();

        if accumulator >= 1.0 / 120.0 {
            if handles.is_mouse_down() {
                sim.add_object(PointMass::new(
                    sim.window().screen_to_simulation(handles.get_mouse_position()),
                    rand::rng().random_range(1.0..2.0),
                    1.0,
                    false));
            }
            sim.update();
            {
                let mut draw = handles.begin_drawing();
                draw.clear_background(Color::RAYWHITE);
                draw.draw_simulation(&sim);
                draw.draw_fps(10, 10);
            }
            accumulator = 0.0;
        }
    }
}
