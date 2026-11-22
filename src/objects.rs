use na::Vector2;
use raylib::core::color::Color;
use rand::prelude::*;

#[derive(Debug)]
pub struct PointMass {
    pub position: Vector2<f32>, // Meters
    previous: Vector2<f32>,     // Meters
    acceleration: Vector2<f32>, // Meters per Second squared
    radius: f32,                // Meters
    mass: f32,                  // Kilograms
    pub color: Color,
    pinned: bool,
}

impl PointMass {
    pub fn new(position: Vector2<f32>, radius: f32, mass: f32, pinned: bool) -> PointMass {
        PointMass {
            position,
            previous: position,
            acceleration: Vector2::zeros(),
            radius,
            mass,
            color: Color::color_from_hsv(rand::rng().random_range(0.0..=360.0), 1.0, 1.0),
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

    pub fn update_position(&mut self, delta: f32) {
        if self.pinned { 
            self.position = self.previous;
            return; 
        }
        let velocity = self.position - self.previous;
        self.previous = self.position;
        self.position = self.position + velocity + self.acceleration.scale(delta * delta);
        self.acceleration = Vector2::<f32>::zeros();
        if velocity.magnitude() > 100.0 { self.previous = self.position;}
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration += force.scale(1.0 / self.mass);
    }

    pub fn velocity(&self) -> Vector2<f32> {
        self.position - self.previous
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }
}

pub struct Link {
    object_1: usize,
    object_2: usize,
    length: f32,
}

impl Link {
    pub fn new(
        object_1: usize,
        object_2: usize,
        length: f32) -> Link {
        Link {
            object_1,
            object_2,
            length,
        }
    }
    pub fn apply(&mut self, objects: &mut Vec<PointMass>) {
        let mut object_1 = objects.get(self.object_1).expect("Object not in simulation!").position;
        let mut object_2 = objects.get(self.object_2).expect("Object not in simulation!").position;
        
        let axis = object_1 - object_2;
        let delta = axis.magnitude() - self.length;
        object_1 -= axis.normalize().scale(0.5 * delta);
        object_2 += axis.normalize().scale(0.5 * delta);

        objects[self.object_1].position = object_1;
        objects[self.object_2].position = object_2;
    }
}
