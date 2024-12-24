mod vector;
mod ball;
mod circle_structure;
mod simulation;

use ggez::{event, ContextBuilder,conf};
use simulation::Simulation;

fn main() -> ggez::GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Circular Simulation", "Akim")
        
        .window_mode(conf::WindowMode::default().dimensions(800.0,600.0).resizable(true))
        .build()
        .unwrap();

    let mut simulation = Simulation::new();
    event::run(ctx, event_loop,  simulation)
}
