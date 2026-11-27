use na::Vector2;
use raylib::core::color::Color;
use rand::prelude::*;


pub trait Collidable {
    fn update_position(&mut self, delta: f32);
    fn apply_force(&mut self, force: Vector2<f32>);
    fn apply_acceleration(&mut self, acceleration: Vector2<f32>);

    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, position: Vector2<f32>);
}

pub fn resolve_collision(a: &mut Object, b: &mut Object) {
    match (a, b) {
        (Object::PointMass(object_a), Object::PointMass(object_b)) => {
            let collision_axis = object_a.get_position() - object_b.get_position();
            let delta = object_a.get_radius() + object_b.get_radius() - collision_axis.magnitude();
            let mass_total = object_a.get_mass() + object_b.get_mass();
            if collision_axis.magnitude() == 0.0 {
                let old = object_a.get_position();
                object_a.set_position(old + Vector2::new(1.0, 0.0).scale(object_b.get_mass() / mass_total * delta));
                let old = object_b.get_position();
                object_b.set_position(old - Vector2::new(1.0, 0.0).scale(object_a.get_mass() / mass_total * delta));
            } else {
                let old = object_a.get_position();
                object_a.set_position(old + collision_axis.normalize().scale(object_b.get_mass() / mass_total * delta));
                let old = object_b.get_position();
                object_b.set_position(old - collision_axis.normalize().scale(object_a.get_mass() / mass_total * delta));
            }
        },
    }
}

pub fn is_colliding(a: & Object, b: &Object) -> bool {
    match (a, b) {
        (Object::PointMass(object_a), Object::PointMass(object_b)) => {
            let collision_axis = object_a.get_position() - object_b.get_position();
            collision_axis.magnitude() < object_a.get_radius() + object_b.get_radius()
        },
    }
}

#[derive(Debug)]
pub enum Object {
    PointMass(PointMass),
}

impl Collidable for Object {
    fn update_position(&mut self, delta: f32) { 
        match self { 
            Object::PointMass(object) => object.update_position(delta), 
        }
    }
    fn apply_force(&mut self, force: Vector2<f32>) { 
        match self { 
            Object::PointMass(object) => object.apply_force(force), 
        }
    }
    fn apply_acceleration(&mut self, acceleration: Vector2<f32>) { 
        match self { 
            Object::PointMass(object) => object.apply_acceleration(acceleration), 
        }
    }

    fn get_position(&self) -> Vector2<f32> { 
        match self { 
            Object::PointMass(object) => object.get_position(), 
        }
    }
    fn set_position(&mut self, position: Vector2<f32>) { 
        match self { 
            Object::PointMass(object) => object.set_position(position), 
        }
    }
}


#[derive(Debug)]
pub enum WorldBoundary {
    Circle { center: Vector2<f32>, radius: f32 },
    Box { bottom_left: Vector2<f32>, dimensions: Vector2<f32> },
}

impl WorldBoundary {
    pub fn apply_boundary(&self, object: &mut Object) {
        match self {
            WorldBoundary::Circle { center, radius } => {
                match object {
                    Object::PointMass(point_mass) => {
                        let diff = point_mass.get_position() - center;
                        if diff.magnitude() > radius - point_mass.get_radius() {
                            point_mass.set_position(diff.normalize().scale(radius - point_mass.get_radius()) + center);
                        }
                    },
                }
            },
            WorldBoundary::Box { bottom_left, dimensions } => {
                match object {
                    Object::PointMass(point_mass) => {
                        if point_mass.get_position().x - point_mass.get_radius() < bottom_left.x {
                            point_mass.set_position(Vector2::<f32>::new(bottom_left.x + point_mass.get_radius(), point_mass.get_position().y));
                        } else if point_mass.get_position().x + point_mass.get_radius() > bottom_left.x + dimensions.x {
                            point_mass.set_position(Vector2::<f32>::new(bottom_left.x + dimensions.x - point_mass.get_radius(), point_mass.get_position().y));
                        }
                        if point_mass.get_position().y - point_mass.get_radius() < bottom_left.y {
                            point_mass.set_position(Vector2::<f32>::new(point_mass.get_position().x, bottom_left.y + point_mass.get_radius()));
                        } else if point_mass.get_position().y + point_mass.get_radius() > bottom_left.y + dimensions.y {
                            point_mass.set_position(Vector2::<f32>::new(point_mass.get_position().x, bottom_left.y + dimensions.y - point_mass.get_radius()));
                        }
                    },
                }
            },
        }
    }
}


#[derive(Debug)]
pub struct PointMass {
    position: Vector2<f32>,     // Meters
    previous: Vector2<f32>,     // Meters
    acceleration: Vector2<f32>, // Meters per Second squared
    radius: f32,                // Meters
    mass: f32,                  // Kilograms
    pub color: Color,
    pinned: bool,
}

impl Collidable for PointMass {
    fn update_position(&mut self, delta: f32) {
        if self.pinned { 
            self.position = self.previous;
            return; 
        }
        let velocity = self.position - self.previous;
        self.previous = self.position;
        self.position = self.position + velocity + self.acceleration.scale(delta * delta);
        self.acceleration = Vector2::<f32>::zeros();
    }
    fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration += force.unscale(self.mass);
    }
    fn apply_acceleration(&mut self, acceleration: Vector2<f32>) {
        self.acceleration += acceleration;
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }
}

impl PointMass {
    pub fn new(position: Vector2<f32>, radius: f32, mass: f32, pinned: bool) -> PointMass {
        PointMass {
            position,
            previous: position,
            acceleration: Vector2::zeros(),
            radius,
            mass,
            color: Color::BLUE.lerp(Color::RED, (mass - 1.0) / 9.0),
            pinned,
        }
    }
    pub fn with(
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        radius: f32,
        mass: f32,
        color: Color,
        pinned: bool) -> PointMass {
        PointMass {
            position,
            previous: position - velocity,
            acceleration: Vector2::zeros(),
            radius,
            mass,
            color,
            pinned,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }
}

#[derive(Debug)]
pub struct RigidBody {
    position: Vector2<f32>,     // Meters
    previous: Vector2<f32>,     // Meters
    acceleration: Vector2<f32>, // Meters per Second squared
    
    edges: usize,
    midpoints: Vec<Vector2<f32>>,
    normals: Vec<Vector2<f32>>,

    mass: f32,                  // Kilograms
    pub color: Color,
    pinned: bool,
}

impl Collidable for RigidBody {
    fn update_position(&mut self, delta: f32) {
        if self.pinned { 
            self.position = self.previous;
            return; 
        }
        let velocity = self.position - self.previous;
        self.previous = self.position;
        self.position = self.position + velocity + self.acceleration.scale(delta * delta);
        self.acceleration = Vector2::<f32>::zeros();
        let delta = self.position - self.previous;
        for midpoint in &mut self.midpoints {
            *midpoint += delta;
        }
    }
    fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration += force.unscale(self.mass);
    }
    fn apply_acceleration(&mut self, acceleration: Vector2<f32>) {
        self.acceleration += acceleration;
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }
}
