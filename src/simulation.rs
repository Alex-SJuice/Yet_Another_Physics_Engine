pub mod rendering;

use std::time::{Instant, Duration};
use rendering::{Window, Handles};
use na::Vector2;


pub struct TimeManager {
	frame_time: Instant,
	delta: Duration,
}

impl TimeManager {
	pub fn initialize() -> TimeManager {
		TimeManager {
			frame_time: Instant::now(),
			delta: Duration::ZERO,
		}
	}

	pub fn delta(&self) -> Duration {
		self.delta
	}

	pub fn update(&mut self) {
		self.delta = self.frame_time.elapsed();
        self.frame_time = Instant::now();
	}
}

// pub struct Simulation {
// 	window: Window,
// 	pub time_manager: TimeManager,
// }

pub struct SimulationBuilder<'a> {
	name: &'a str,
	position: Vector2<f32>,	// Simulation space coordinate of the window's top left corner
	scale: Vector2<f32>,		// Size of the window in the simulation
	window: Vector2<f32>,	// Size of the window on screen
}

pub fn initialize<'a>() -> SimulationBuilder<'a> {
	SimulationBuilder {
		name: "",
		position: Vector2::zeros(),
		scale: Vector2::zeros(),
		window: Vector2::zeros(),
	}	
}

impl<'a> SimulationBuilder<'a> {
	pub fn title(&mut self, name: &'a str) -> &mut Self{
		self.name = name;
		return self;
	}

	pub fn window_position(&mut self, x: f32, y: f32) -> &mut Self {
		self.position = Vector2::new(x, y);
		return self;
	}

	pub fn window_scale(&mut self, x: f32, y: f32) -> &mut Self {
		self.scale = Vector2::new(x, y);
		return self;
	}

	pub fn window_size(&mut self, x: i32, y: i32) -> &mut Self {
		self.window = Vector2::new(x as f32, y as f32);
		return self;
	}

	pub fn build(&self) -> (Handles, Window, TimeManager) {
		let (handle, thread) = raylib::init()
			.size(self.window.x as i32, self.window.y as i32)
			.title(self.name)
			.build();
		(
			Handles::new(
				handle,
				thread,
			),
			Window::new(
				self.position,
				self.scale,
				self.window,
			),
			TimeManager::initialize()
		)
	}
}
