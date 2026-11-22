use super::super::objects::*;
use raylib::prelude::*;
use na::Vector2;

pub trait ObjectDraw {
	fn draw_pointmass(&mut self, window: &Window, object: &PointMass);
}

impl<T: RaylibDraw> ObjectDraw for T {
	fn draw_pointmass(&mut self, window: &Window, object: &PointMass) {
		let position = window.simulation_to_screen(object.position());
		let shape = window.scale().scale(object.radius());
		self.draw_ellipse(
			position.x as i32, 
			position.y as i32, 
			shape.x, 
			shape.y, 
			Color::WHITE);
	}
}

pub struct Handles {
	handle: RaylibHandle,
	thread: RaylibThread,
}

impl Handles {
	pub fn new(handle: RaylibHandle, thread: RaylibThread) -> Handles {
		Handles{
			handle,
			thread,
		}
	}

	pub fn begin_drawing<'a>(&'a mut self) -> RaylibDrawHandle<'a> {
		self.handle.begin_drawing(&self.thread)
	}

	pub fn window_should_close(&self) -> bool {
		self.handle.window_should_close()
	}

	pub fn delta(&self) -> f32 {
		self.handle.get_frame_time()
	}
}

pub struct Window {
	position: Vector2<f32>,	// Simulation space coordinate of the window's top left corner
	scale: Vector2<f32>,		// Size of the window in the simulation
	window: Vector2<f32>,		// Size of the window on screen
}

impl Window {
	pub fn new(
		position: Vector2<f32>,
		scale: Vector2<f32>,
		window: Vector2<f32>
		) -> Window {
		Window {
			position,
			scale,
			window,
		}
	}

	pub fn scale(&self) -> Vector2<f32> {
		self.window.component_div(&self.scale)
	}
	pub fn simulation_to_screen(&self, position: Vector2<f32>) -> Vector2<f32> {
		let mut pos = position; 
		pos.x -= self.position.x;
		pos.y -= self.position.y;
		pos.x *= self.window.x / self.scale.x;
		pos.y *= self.window.y / self.scale.y;
		pos.y *= -1.0;
		pos
	}
}