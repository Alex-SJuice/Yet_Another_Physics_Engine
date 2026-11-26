extern crate nalgebra as na;

mod simulation;
mod objects;

use objects::{PointMass, Object, Link, SoftBody};
use simulation::{TimeManager, rendering::ObjectDraw};
use na::{Vector2};
use raylib::prelude::*;
use rand::prelude::*;
use rand::prelude::*;
use raylib::consts::MouseButton;

fn main() {
    let (mut handles, mut sim) = simulation::initialize()
        .title("Simulation")
        .window_size(500, 500)
        .window_scale(100.0, 100.0)
        .window_position(-50.0, 100.0)
        .circle_boundary(Vector2::<f32>::new(0.0, 50.0), 50.0)
        .build();

    let mut render_timer = TimeManager::initialize();
    let mut accumulator = 0.0;

    while !handles.window_should_close() {
        render_timer.update();
        accumulator += render_timer.delta().as_secs_f32();

        if accumulator >= 1.0 / 120.0 {
            if handles.is_mouse_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                let mass = rand::rng().random_range(1.0..10.0);
                // sim.add_object(Object::PointMass(PointMass::new(
                //         sim.window().screen_to_simulation(handles.get_mouse_position()),// + Vector2::<f32>::new(i as f32 - cubes as f32, i as f32 - cubes as f32),
                //         3.0,
                //         mass,
                //         false)));
                let mut object = SoftBody::new();
                for i in 0..4 {
                    object.add_node(PointMass::new(
                        sim.window().screen_to_simulation(handles.get_mouse_position()) + Vector2::<f32>::new(i as f32, i as f32),
                        3.0,
                        mass,
                        false
                    ));
                }
                object.add_edge(Link::new(0, 1, 6.0, 1.0));
                object.add_edge(Link::new(1, 2, 6.0, 1.0));
                object.add_edge(Link::new(2, 3, 6.0, 1.0));
                object.add_edge(Link::new(3, 0, 6.0, 1.0));
                object.add_edge(Link::new(0, 2, 6.0 * 2.0_f32.sqrt(), 1.0));
                object.add_edge(Link::new(1, 3, 6.0 * 2.0_f32.sqrt(), 1.0));
                sim.add_object(Object::SoftBody(object));
            }
            sim.update(&handles);
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
