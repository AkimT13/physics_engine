use crate::vector::Vector2;

pub struct Ball {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.0, 0.0),
            radius,
        }
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.velocity.y += 9.8 * dt; // Gravity
    }

    pub fn update_position(&mut self, dt: f32) {
        self.position = self.position.add(&self.velocity.scale(dt));
    }
}
