pub mod rendering;

use std::time::{Instant, Duration};
use rendering::{Window, Handles};
use super::objects::{PointMass, Object, Collidable, WorldBoundary, resolve_collision, is_colliding};
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
	grid: Vec<Vec<usize>>,
	grid_size: Vector2<usize>,
	grid_offset: Vector2<f32>,
	object_size: f32,
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
			//self.apply_grab(handle);
			self.update_positions(self.time_manager.delta().as_secs_f32() / substeps as f32);
		}
	}

	pub fn update_positions(&mut self, delta: f32) {
		for object in &mut self.objects {
			object.update_position(delta);
		}
		for i in 0..self.grid.len() {
			self.grid[i] = Vec::new();
		}
		for object in 0..self.objects.len() {
			let x = ((self.objects[object].get_position().x - self.grid_offset.x) / self.object_size).floor() as usize;
			let y = ((self.objects[object].get_position().y - self.grid_offset.y) / self.object_size).floor() as usize;
			self.grid[y + x * self.grid_size.y].push(object);
		}
	}

	pub fn apply_gravity(&mut self, gravity: Vector2<f32>) {
		for object in &mut self.objects {
			match object {
				Object::PointMass(point_mass) => {
					point_mass.apply_force(gravity * point_mass.get_mass());
				},
			}	
		}
	}

	pub fn apply_constraint(&mut self) {
		for object in &mut self.objects {
			self.constraint.apply_boundary(object);
		}
	}

	// pub fn apply_grab(&mut self, handle: &Handles) -> bool {
	// 	if !handle.is_mouse_down(MouseButton::MOUSE_BUTTON_LEFT) { 
	// 		self.grabbed = self.objects.len();
	// 		return false;
	// 	}
	// 	let mouse = self.window().screen_to_simulation(handle.get_mouse_position());
	// 	if self.grabbed < self.objects.len() {
	// 		self.objects[self.grabbed].set_position(mouse);
	// 		return true;
	// 	}
	// 	for i in 0..self.objects.len() {
	// 		if self.objects[i].is_colliding(&Object::PointMass(PointMass::new(mouse, 0.0, 0.0, false))) {
	// 			self.grabbed = i as usize;
	// 			self.objects[self.grabbed].set_position(mouse);
	// 			return true;
	// 		}
	// 	}
	// 	self.grabbed = self.objects.len();
	// 	return false;
	// }

	pub fn solve_collisions(&mut self) {
		if self.objects.len() <= 0 {
			return;
		}

		// Broad Phase
		let mut pairs: Vec<(usize, usize)> = Vec::new();
		for i in 0..self.grid_size.x {
			for j in 0..self.grid_size.y {
				let index_a = j + i * self.grid_size.y; 
				if self.grid[index_a].len() == 0 { continue; }
				for object_a in 0..self.grid[index_a].len() - 1 {
					for object_b in object_a + 1..self.grid[index_a].len() {
						pairs.push((self.grid[index_a][object_a], self.grid[index_a][object_b]));
					}
				}
				for dx in 0..=1 {
					if i == self.grid_size.x - 1 && dx == 1 { continue; }
					for dy in -1..=1 {
						if j == 0 && dy == -1isize { continue; }
						if j == self.grid_size.y - 1 && dy == 1 { continue; }
						if dx == 0 && (dy == 0 || dy == -1) { continue; }
						let index_b = (j as isize + dy + (i as isize + dx as isize) * self.grid_size.y as isize) as usize;
						for object_a in 0..self.grid[index_a].len() {
							for object_b in 0..self.grid[index_b].len() {
								pairs.push((self.grid[index_a][object_a], self.grid[index_b][object_b]));
							}
						}
					}
				}
			}
		}

		// Narrow Phase
		for (object_a, object_b) in pairs {
			if object_a == object_b { continue; }
			let min = object_a.min(object_b);
			let max = object_a.max(object_b);
			let (left, right) = &mut self.objects.split_at_mut(min + 1);
			if is_colliding(&left[min], &right[max - min - 1]) {
				resolve_collision(&mut left[min], &mut right[max - min - 1]);	
			}
		}
	}

	pub fn new(window: Window, constraint: WorldBoundary, object_size: f32) -> Simulation {
		let mut grid = Vec::new();
		let mut grid_offset = Vector2::<f32>::zeros();
		let mut grid_size = Vector2::<usize>::zeros();
		match constraint {
			WorldBoundary::Circle{ center, radius } => {
				grid_offset = Vector2::new(center.x - radius, center.y - radius);
				grid_size = Vector2::new((radius * 2.0 / object_size).ceil() as usize, (radius * 2.0 / object_size).ceil() as usize);
				for i in 0..(radius * 2.0 / object_size).ceil() as usize * (radius * 2.0 / object_size).ceil() as usize {
					grid.push(Vec::new());
				}
			}, 
			WorldBoundary::Box { bottom_left, dimensions } => {
				grid_offset = bottom_left;
				grid_size = Vector2::<usize>::new((dimensions.x / object_size).ceil() as usize, (dimensions.y / object_size).ceil() as usize);
				for i in 0..(dimensions.x / object_size).ceil() as usize * (dimensions.y / object_size).ceil() as usize {
					grid.push(Vec::new());
				}
			},
		}

		Simulation {
			window,
			time_manager: TimeManager::initialize(),
			objects: Vec::new(),
			grid,
			grid_size,
			grid_offset,
			object_size,
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
		constraint: WorldBoundary::Circle{ center: Vector2::<f32>::zeros(), radius: 0.0 },
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
				self.constraint,
				2.0
			)
		)
	}
}
