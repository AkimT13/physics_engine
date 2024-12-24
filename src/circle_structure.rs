use crate::vector::Vector2;
use crate::ball::Ball;

pub struct Layer {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub is_active: bool,
    pub thickness: f32
}

pub struct CircleStructure {
    pub center: Vector2,
    pub layers: Vec<Layer>,
    pub rotation_angle: f32,
    pub rotation_speed: f32,
}

impl CircleStructure {
    pub fn new(center: Vector2, num_layers: usize, layer_width: f32, rotation_speed: f32) -> Self {
        let mut layers = Vec::new();
        for i in 0..num_layers {
            layers.push(Layer {
                inner_radius: i as f32 * layer_width,
                outer_radius: (i + 1) as f32 * layer_width,
                is_active: true,
                thickness: 200.0
            });
        }

        Self {
            center,
            layers,
            rotation_angle: 0.0,
            rotation_speed,
        }
    }

    pub fn rotate(&mut self, dt: f32) {
        self.rotation_angle += self.rotation_speed * dt;
        if self.rotation_angle > 360.0 {
            self.rotation_angle -= 360.0;
        }
    }

    pub fn check_collision_with_ball(&mut self, ball: &mut Ball) {
        let distance = ((ball.position.x - self.center.x).powi(2)
            + (ball.position.y - self.center.y).powi(2))
        .sqrt();

        for layer in &mut self.layers {
            if layer.is_active
                && distance >= layer.inner_radius
                && distance <= layer.outer_radius
            {
                // Ball hit this layer
                layer.is_active = false;

                // Reverse the ball's velocity
                ball.velocity.x *= -1.0;
                ball.velocity.y *= -1.0;

                break; // Only one collision per frame
            }
        }
    }
}
