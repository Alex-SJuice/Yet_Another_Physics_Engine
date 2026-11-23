pub mod rendering;

use std::time::{Instant, Duration};
use rendering::{Window, Handles};
use super::objects::{PointMass, Link};
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

pub struct Simulation {
	window: Window,
	time_manager: TimeManager,
	objects: Vec<PointMass>,
	links: Vec<Link>,
	constraint_position: Vector2<f32>,
	constraint_radius: f32,
}

impl Simulation {

	pub fn update(&mut self) {
		let substeps = 8;
		self.time_manager.update();
		for _ in 0..substeps {
			self.apply_gravity(Vector2::<f32>::new(0.0, -100.0));
			self.apply_links();
			self.apply_constraint(self.constraint_position, self.constraint_radius);
			self.solve_collisions();
			self.update_positions(self.time_manager.delta().as_secs_f32() / substeps as f32);
		}
	}

	pub fn update_positions(&mut self, delta: f32) {
		for object in &mut self.objects {
			object.update_position(delta);
		}
	}

	pub fn apply_gravity(&mut self, gravity: Vector2<f32>) {
		for object in &mut self.objects {
			object.apply_force(gravity * object.mass());
		}
	}

	pub fn apply_constraint(&mut self, position: Vector2<f32>, radius: f32) {
		for object in &mut self.objects {
			let diff = object.position - position;
			if diff.magnitude() > radius - object.radius() {
				object.position = diff.normalize().scale(radius - object.radius()) + position;
			}
		}
	}

	pub fn solve_collisions(&mut self) {
		if self.objects.len() <= 0 {
			return;
		}
		let objects = &mut self.objects;
		for i in 0..objects.len() - 1 {
			let mut current = objects[i].position;
			for j in i + 1..objects.len() {
				let mut other = objects[j].position;
				let collision_axis = current - other;
				if collision_axis.magnitude() < objects[i].radius() + objects[j].radius() {
					let delta = objects[i].radius() + objects[j].radius() - collision_axis.magnitude();
					let mass_total = objects[j].mass() + objects[i].mass();
					if collision_axis.magnitude() == 0.0 {
						current += Vector2::new(1.0, 0.0).scale(objects[j].mass() / mass_total * delta);
						other -= Vector2::new(1.0, 0.0).scale(objects[i].mass() / mass_total * delta);
					} else {
						current += collision_axis.normalize().scale(objects[j].mass() / mass_total * delta);
						other -= collision_axis.normalize().scale(objects[i].mass() / mass_total * delta);
					}
					objects[j].position = other;
					objects[i].position = current;
				}
			}
		}
	}

	pub fn apply_links(&mut self) {
		if self.links.len() <= 0 { return; }
		for link in &mut self.links {
			link.apply(&mut self.objects);
		}
	}

	pub fn new(window: Window) -> Simulation {
		Simulation {
			window,
			time_manager: TimeManager::initialize(),
			objects: Vec::new(),
			links: Vec::new(),
			constraint_position: Vector2::<f32>::zeros(),
			constraint_radius: 0.0,
		}
	}

	pub fn set_constraint(&mut self, position: Vector2<f32>, radius: f32) {
		self.constraint_radius = radius;
		self.constraint_position = position;
	}

	pub fn add_object(&mut self, object: PointMass) {
		self.objects.push(object);
	}

	pub fn add_link(&mut self, index_1: usize, index_2: usize, length: f32) {
		self.links.push(
			Link::new(
				index_1,
				index_2,
				length));
	}

	pub fn constraint(&self) -> (Vector2<f32>, f32) {
		(self.constraint_position, self.constraint_radius)
	}

	pub fn objects<'a>(&'a self) -> &'a Vec<PointMass> {
		&self.objects
	}

	pub fn window<'a>(&'a self) -> &'a Window {
		&self.window
	}
}


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

	pub fn build(&self) -> (Handles, Simulation) {
		let (handle, thread) = raylib::init()
			.size(self.window.x as i32, self.window.y as i32)
			.title(self.name)
			.msaa_4x()
			.vsync()
			.build();
		(
			Handles::new(
				handle,
				thread,
			),
			Simulation::new(
				Window::new(
					self.position,
					self.scale,
					self.window,
				)
			)
		)
	}
}
