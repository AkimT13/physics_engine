use crate::ball::Ball;
use crate::circle_structure::CircleStructure;
use crate::vector::Vector2;
use ggez::{event, graphics, Context, GameResult};

pub struct Simulation {
    pub ball: Ball,
    pub circle: CircleStructure,
}

impl Simulation {
    pub fn new() -> Self {
        let ball = Ball::new(300.0, 200.0, 10.0);
        let circle = CircleStructure::new(Vector2::new(300.0, 300.0), 5, 30.0, 0.5);

        Self { ball, circle }
    }
}

impl event::EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let dt = 0.016; // Fixed time step (~60 FPS)
        self.ball.apply_gravity(dt);
        self.ball.update_position(dt);

        self.circle.check_collision_with_ball(&mut self.ball);
        self.circle.rotate(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        // Draw the ball
        let ball_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.ball.position.x, self.ball.position.y],
            self.ball.radius,
            0.1,
            graphics::Color::YELLOW,
        )?;
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

        // Draw the circle structure
        for layer in &self.circle.layers {
            if layer.is_active {
                let mesh = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::stroke(2.0),
                    [self.circle.center.x, self.circle.center.y],
                    layer.outer_radius,
                    0.1,
                    graphics::Color::WHITE,
                )?;
                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
