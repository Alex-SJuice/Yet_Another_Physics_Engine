pub mod rendering;

use std::time::{Instant, Duration};
use rendering::{Window, Handles};
use super::objects::{PointMass, Link, Object, Collidable, WorldBoundary};
use na::Vector2;
use raylib::consts::MouseButton;


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
	objects: Vec<Object>,
	constraint: WorldBoundary,
	grabbed: usize,

}
 
impl Simulation {

	pub fn update(&mut self, handle: &Handles) {
		let substeps = 8;
		self.time_manager.update();
		for _ in 0..substeps {
			self.apply_gravity(Vector2::<f32>::new(0.0, -100.0));
			self.apply_constraint();
			self.solve_collisions();
			self.apply_grab(handle);
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
			match object {
				Object::PointMass(point_mass) => {
					point_mass.apply_force(gravity * point_mass.get_mass());
				},
				Object::SoftBody(soft_body) => {
					for node in soft_body.get_nodes_mut() {
						if let Object::PointMass(point_mass) = node {
							point_mass.apply_force(gravity * point_mass.get_mass());
						}
					}
				},
			}	
		}
	}

	pub fn apply_constraint(&mut self) {
		for object in &mut self.objects {
			self.constraint.apply_boundary(object);
		}
	}

	pub fn apply_grab(&mut self, handle: &Handles) -> bool {
		if !handle.is_mouse_down(MouseButton::MOUSE_BUTTON_LEFT) { 
			self.grabbed = self.objects.len();
			return false;
		}
		let mouse = self.window().screen_to_simulation(handle.get_mouse_position());
		if self.grabbed < self.objects.len() {
			self.objects[self.grabbed].set_position(mouse);
			return true;
		}
		for i in 0..self.objects.len() {
			if self.objects[i].is_colliding(&Object::PointMass(PointMass::new(mouse, 0.0, 0.0, false))) {
				self.grabbed = i as usize;
				self.objects[self.grabbed].set_position(mouse);
				return true;
			}
		}
		self.grabbed = self.objects.len();
		return false;
	}

	pub fn solve_collisions(&mut self) {
		if self.objects.len() <= 0 {
			return;
		}
		let objects = &mut self.objects;
		for i in 0..objects.len() - 1 {
			let (left, right) = objects.split_at_mut(i + 1);
			for j in 0..right.len() {
				if left[i].is_colliding(&right[j]) {
					left[i].resolve_collision(&mut right[j]);
				}
			}
			if let Object::SoftBody(soft_body) = &mut objects[i] {
				soft_body.self_collision();
			}
		}
	}

	pub fn new(window: Window, constraint: WorldBoundary) -> Simulation {
		Simulation {
			window,
			time_manager: TimeManager::initialize(),
			objects: Vec::new(),
			constraint,
			grabbed: 0,
		}
	}

	
	pub fn get_objects<'a>(&'a self) -> &'a Vec<Object> {
		&self.objects
	}
	pub fn get_objects_mut<'a>(&'a mut self) -> &'a mut Vec<Object> {
		&mut self.objects
	}

	pub fn add_object(&mut self, object: Object) {
		self.objects.push(object);
	}

	pub fn set_constraint(&mut self, constraint: WorldBoundary) {
		self.constraint = constraint;
	}

	pub fn get_constraint<'a>(&'a self) -> &'a WorldBoundary {
		&self.constraint
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
	constraint: WorldBoundary,
}

pub fn initialize<'a>() -> SimulationBuilder<'a> {
	SimulationBuilder {
		name: "",
		position: Vector2::zeros(),
		scale: Vector2::zeros(),
		window: Vector2::zeros(),
		constraint: WorldBoundary::None,
	}	
}

impl<'a> SimulationBuilder<'a> {
	pub fn title(mut self, name: &'a str) -> Self{
		self.name = name;
		return self;
	}

	pub fn window_position(mut self, x: f32, y: f32) -> Self {
		self.position = Vector2::new(x, y);
		return self;
	}

	pub fn window_scale(mut self, x: f32, y: f32) -> Self {
		self.scale = Vector2::new(x, y);
		return self;
	}

	pub fn window_size(mut self, x: i32, y: i32) -> Self {
		self.window = Vector2::new(x as f32, y as f32);
		return self;
	}

	pub fn circle_boundary(mut self, center: Vector2<f32>, radius: f32) -> Self {
		self.constraint = WorldBoundary::Circle{ center, radius, };
		return self;
	}

	pub fn box_boundary(mut self, bottom_left: Vector2<f32>, dimensions: Vector2<f32>) -> Self {
		self.constraint = WorldBoundary::Box{ bottom_left, dimensions, };
		return self;
	}

	pub fn build(self) -> (Handles, Simulation) {
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
				),
				self.constraint
			)
		)
	}
}
