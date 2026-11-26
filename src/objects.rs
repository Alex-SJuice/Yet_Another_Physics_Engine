use na::Vector2;
use raylib::core::color::Color;
use rand::prelude::*;


pub trait Collidable {
    fn update_position(&mut self, delta: f32);
    fn apply_force(&mut self, force: Vector2<f32>);
    
    fn resolve_collision(&mut self, other: &mut Object);
    fn is_colliding(&self, other: &Object) -> bool;

    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, position: Vector2<f32>);

    fn get_velocity(&self) -> Vector2<f32>;
}

#[derive(Debug)]
pub enum Object {
    PointMass(PointMass),
    SoftBody(SoftBody),
}

impl Collidable for Object {
    fn update_position(&mut self, delta: f32)             { 
        match self { 
            Object::PointMass(object) => object.update_position(delta), 
            Object::SoftBody(object) => object.update_position(delta),
        }
    }
    fn apply_force(&mut self, force: Vector2<f32>)        { 
        match self { 
            Object::PointMass(object) => object.apply_force(force), 
            Object::SoftBody(object) => object.apply_force(force),
        }
    }
    
    fn resolve_collision(&mut self, other: &mut Object)   { 
        match self { 
            Object::PointMass(object) => object.resolve_collision(other), 
            Object::SoftBody(object) => object.resolve_collision(other)
        }
    }
    fn is_colliding(&self, other: &Object) -> bool        { 
        match self { 
            Object::PointMass(object) => object.is_colliding(other), 
            Object::SoftBody(object) => object.is_colliding(other),
        }
    }

    fn get_position(&self) -> Vector2<f32>                { 
        match self { 
            Object::PointMass(object) => object.get_position(), 
            Object::SoftBody(object) => object.get_position(),
        }
    }
    fn set_position(&mut self, position: Vector2<f32>)                                { 
        match self { 
            Object::PointMass(object) => object.set_position(position), 
            Object::SoftBody(object) => object.set_position(position),
        }
    }

    fn get_velocity(&self) -> Vector2<f32>                { 
        match self { 
            Object::PointMass(object) => object.get_position(), 
            Object::SoftBody(object) => object.get_position(),
        }
    }
}


#[derive(Debug)]
pub enum WorldBoundary {
    Circle { center: Vector2<f32>, radius: f32 },
    Box { bottom_left: Vector2<f32>, dimensions: Vector2<f32> },
    None,
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
                    Object::SoftBody(soft_body) => {
                        for object in soft_body.get_nodes_mut() {
                            self.apply_boundary(object);
                        }
                        soft_body.recalculate_bound();
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
                    Object::SoftBody(soft_body) => {
                        for object in soft_body.get_nodes_mut() {
                            self.apply_boundary(object);
                        }
                        soft_body.recalculate_bound();
                    },
                }
            },
            WorldBoundary::None => {},
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
        self.acceleration += force.scale(1.0 / self.mass);
    }

    fn resolve_collision(&mut self, other: &mut Object) {
        match other {
            Object::PointMass(object) => {
                let collision_axis = self.position - object.get_position();
                let delta = self.radius + object.get_radius() - collision_axis.magnitude();
                let mass_total = self.mass + object.get_mass();

                if collision_axis.magnitude() == 0.0 {
                    self.position += Vector2::new(1.0, 0.0).scale(object.get_mass() / mass_total * delta);
                    let old = object.get_position();
                    object.set_position(old - Vector2::new(1.0, 0.0).scale(self.mass / mass_total * delta));
                } else {
                    self.position += collision_axis.normalize().scale(object.get_mass() / mass_total * delta);
                    let old = object.get_position();
                    object.set_position(old - collision_axis.normalize().scale(self.mass / mass_total * delta));
                }
            },
            Object::SoftBody(object) => {
                for node in object.get_nodes_mut() {
                    self.resolve_collision(node);
                }
                object.recalculate_bound();
            },
        }
    }
    fn is_colliding(&self, other: &Object) -> bool {
        match other {
            Object::PointMass(object) => {
                let collision_axis = self.position - object.get_position();
                collision_axis.magnitude() < self.radius + object.get_radius()
            },
            Object::SoftBody(object) => {
                for node in object.get_nodes() {
                    if self.is_colliding(node) {
                        return true;
                    }
                }
                return false;
            },
        }
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }
    fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }

    fn get_velocity(&self) -> Vector2<f32> {
        self.position - self.previous
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
            color: Color::BLUE.lerp(Color::RED, (mass - 1.0) / 9.0),//Color::color_from_hsv(rand::rng().random_range(0.0..=360.0), 1.0, 1.0),
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
pub struct Link {
    object_1: usize,
    object_2: usize,
    length: f32,
    strength: f32,
}

impl Link {
    pub fn new(
        object_1: usize,
        object_2: usize,
        length: f32,
        strength: f32) -> Link {
        Link {
            object_1,
            object_2,
            length,
            strength,
        }
    }
    pub fn apply(&self, objects: &mut Vec<Object>) {
        let mut object_1 = objects.get(self.object_1).expect("Object not found!").get_position();
        let mut object_2 = objects.get(self.object_2).expect("Object not found!").get_position();
        
        let axis = object_1 - object_2;
        let delta = axis.magnitude() - self.length;
        object_1 -= axis.normalize().scale(0.5 * self.strength * delta);
        object_2 += axis.normalize().scale(0.5 * self.strength * delta);

        objects[self.object_1].set_position(object_1);
        objects[self.object_2].set_position(object_2);
    }
}

#[derive(Debug)]
pub struct SoftBody {
    nodes: Vec<Object>,
    edges: Vec<Link>,
    bound: PointMass,
}

impl Collidable for SoftBody {
    fn update_position(&mut self, delta: f32) {
        for object in &mut self.nodes {
            object.update_position(delta);
        }
        self.apply_links();
        self.recalculate_bound();
    }
     
    fn apply_force(&mut self, force: Vector2<f32>) {
        for object in &mut self.nodes {
            object.apply_force(force);
        }
    }
    
    fn resolve_collision(&mut self, other: &mut Object) {
        for i in 0..self.nodes.len() - 1 {
            self.nodes[i].resolve_collision(other);
        }
        self.recalculate_bound();
    }

    fn is_colliding(&self, other: &Object) -> bool {
        if !self.bound.is_colliding(other) { return false; }
        for object in &self.nodes {
            if object.is_colliding(other) { return true; }
        }
        return false;
    }

    fn get_position(&self) -> Vector2<f32> {
        self.bound.get_position()
    }
    fn set_position(&mut self, position: Vector2<f32>) {
        let mut offsets = Vec::new();
        for i in 0..self.nodes.len() {
            offsets.push(self.get_nodes_mut()[i].get_position() - self.bound.get_position());
        }
        self.bound.set_position(position);
        let bound_position = self.bound.get_position();
        for i in 0..self.nodes.len() {
            self.get_nodes_mut()[i].set_position(bound_position + offsets[i]);
        }
    }

    fn get_velocity(&self) -> Vector2<f32> {
        let mut avg = Vector2::<f32>::zeros();
        for object in &self.nodes {
            avg += object.get_velocity();
        }
        return avg.unscale(self.nodes.len() as f32);
    }
}

impl SoftBody {
    pub fn new() -> SoftBody {
        SoftBody {
            nodes: Vec::new(),
            edges: Vec::new(),
            bound: PointMass::new(Vector2::<f32>::zeros(), 0.0, 0.0, false),
        }
    }

    pub fn recalculate_bound(&mut self) {
        let mut position = Vector2::<f32>::zeros();
        for object in &self.nodes {
            position += object.get_position();
        }
        position.unscale_mut(self.nodes.len() as f32);

        let mut furthest = 0.0;
        for node in &mut self.nodes {
            if let Object::PointMass(object) = node {
                let diff = (object.get_position() - position).magnitude() + object.get_radius();
                if diff > furthest { furthest = diff; }
            }
        }
        self.bound = PointMass::new(position, furthest, 0.0, false);
    }

    pub fn self_collision(&mut self) {
        for i in 0..self.nodes.len() - 1 {
            let (left, right) = self.nodes.split_at_mut(i + 1);
            for j in 0..right.len() {
                if left[i].is_colliding(&right[j]) {
                    left[i].resolve_collision(&mut right[j]);
                }
            }
        }
    }

    pub fn apply_links(&mut self) {
        for link in &self.edges {
            link.apply(&mut self.nodes);
        }
    }

    pub fn add_node(&mut self, node: PointMass) -> &mut Self {
        self.nodes.push(Object::PointMass(node));
        return self;
    }

    pub fn add_edge(&mut self, edge: Link) -> &mut Self {
        self.edges.push(edge);
        return self;
    }

    pub fn get_nodes_mut<'a>(&'a mut self) -> &'a mut Vec<Object> {
        &mut self.nodes
    }

    pub fn get_nodes<'a>(&'a self) -> &'a Vec<Object> {
        &self.nodes
    }

    pub fn get_bound<'a>(&'a self) -> &'a PointMass {
        &self.bound
    }
}