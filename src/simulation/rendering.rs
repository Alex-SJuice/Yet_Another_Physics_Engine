use super::super::objects::PointMass;
use super::super::simulation::Simulation;
use raylib::prelude::*;
use raylib::consts::MouseButton;
use na::Vector2;

pub trait ObjectDraw {
	fn draw_pointmass(&mut self, window: &Window, object: &PointMass);

	fn draw_simulation(&mut self, simulation: &Simulation);
}

impl<T: RaylibDraw> ObjectDraw for T {
	fn draw_pointmass(&mut self, window: &Window, object: &PointMass) {
		let position = window.simulation_to_screen(object.position);
		let shape = window.scale().scale(object.radius());
		self.draw_ellipse(
			position.x as i32, 
			position.y as i32, 
			shape.x, 
			shape.y, 
			object.color);
	}

	fn draw_simulation(&mut self, simulation: &Simulation) {
		let (mut position, radius) = simulation.constraint();
		position = simulation.window().simulation_to_screen(position);
		let radius = simulation.window().scale().scale(radius);
		self.draw_ellipse(
			position.x as i32,
			position.y as i32, 
			radius.x, 
			radius.y,
			Color::BLACK);
		for object in simulation.objects() {
			self.draw_pointmass(simulation.window(), &object);
		}
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

	pub fn get_mouse_position(&self) -> Vector2<f32> {
		Vector2::<f32>::new(self.handle.get_mouse_x() as f32, self.handle.get_mouse_y() as f32)
	}

	pub fn is_mouse_pressed(&self) -> bool {
		self.handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
	}
	pub fn is_mouse_down(&self) -> bool {
		self.handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
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

	pub fn screen_to_simulation(&self, position: Vector2<f32>) -> Vector2<f32> {
		let mut pos = position; 
		pos.y *= -1.0;
		pos.x /= self.window.x / self.scale.x;
		pos.y /= self.window.y / self.scale.y;
		pos.x += self.position.x;
		pos.y += self.position.y;
		pos
	}
}