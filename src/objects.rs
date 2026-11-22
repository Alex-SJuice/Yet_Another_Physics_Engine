use na::Vector2;

#[derive(Debug)]
pub struct PointMass {
    position: Vector2<f32>,     // Meters
    previous: Vector2<f32>,     // Meters
    acceleration: Vector2<f32>, // Meters per Second squared
    elasticity: f32,            //0.0 - 1.0
    radius: f32,                // Meters
    mass: f32,                  // Kilograms
}

impl PointMass {
    pub fn new(position: Vector2<f32>, radius: f32, mass: f32) -> PointMass {
        PointMass {
            position,
            previous: position,
            acceleration: Vector2::zeros(),
            elasticity: 1.0,
            radius,
            mass,
        }
    }
    pub fn with(
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        acceleration: Vector2<f32>,
        elasticity: f32,
        radius: f32,
        mass: f32) -> PointMass {
        PointMass {
            position,
            previous: position - velocity,
            acceleration,
            elasticity,
            radius,
            mass,
        }
    }

    pub fn update_position(&mut self, delta: f32) {
        let last = self.position;
        self.position = last.scale(2.0) - self.previous + self.acceleration.scale(delta * delta);
        self.previous = last;
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration += force.scale(1.0 / self.mass);
    }

    pub fn collide_wall(&mut self, wall: &Wall) -> bool {
        let wall_to_point = (self.position - wall.normal().scale(self.radius)) - wall.position();
        
        if wall_to_point.dot(&wall.normal()) > 0.0 {
            return false;
        } 
        let diff = (wall_to_point - wall.normal().scale(wall_to_point.dot(&wall.normal())) + wall.position()) - (self.position - wall.normal().scale(self.radius));
        //println!("{:?}", wall_to_point - wall.normal().scale(wall_to_point.dot(&wall.normal())) + wall.position());
        self.position += diff;
        self.previous += diff;

        let component = wall.normal().scale(self.velocity().dot(&wall.normal()));
        //println!("{:?}", component);
        self.previous += component + component.scale(self.elasticity);

        return true;
    }

    pub fn collide_floor(&mut self, height: f32) -> bool {
        if self.position.y - self.radius > height {
            return false;
        }
        let diff = height - self.position.y + self.radius;
        self.position.y += diff;
        self.previous.y += diff;
        self.previous.y += (height - self.previous.y) * (self.elasticity);
        return true;
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn velocity(&self) -> Vector2<f32> {
        self.position - self.previous
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

#[derive(Debug)]
pub struct Wall {
    position: Vector2<f32>,
    normal: Vector2<f32>,
}

impl Wall {
    pub fn new(position: Vector2<f32>, normal: Vector2<f32>) -> Wall {
        Wall {
            position,
            normal,
        }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn normal(&self) -> Vector2<f32> {
        self.normal
    }
}