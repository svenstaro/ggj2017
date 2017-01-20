extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics2d;
extern crate ggez;
use ggez::conf;
use ggez::game::{Game, GameState};
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;
use std::time::Duration;

use na::Vector2;
use ncollide::shape::{Plane, Cuboid};
use nphysics2d::world::World;
use nphysics2d::object::{RigidBody, RigidBodyHandle};

struct Block {
    body: RigidBodyHandle<f32>,
}

impl Block {
    fn new(body: RigidBody<f32>, physics_world: &mut World<f32>) -> Block {
        let rb_handle = physics_world.add_rigid_body(body);
        let b = Block { body: rb_handle };
        b
    }
}

struct MainState {
    text: graphics::Text,
    physics_world: World<f32>,
    blocks: Vec<Block>,
}


impl GameState for MainState {
    fn load(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "DejaVuSerif.ttf", 48).unwrap();
        let text = graphics::Text::new(ctx, "Hello world!", &font).unwrap();

        let mut s = MainState {
            text: text,
            physics_world: World::new(),
            blocks: Vec::new(),
        };

        s.physics_world.set_gravity(Vector2::new(0.0, 9.81));

        let static_block = Block::new(RigidBody::new_static(Plane::new(Vector2::new(0.0, -1.0)), 0.3, 0.6), &mut s.physics_world);
        s.blocks.push(static_block);

        let width   = 100;
        let height  = 20;
        let rad     = 0.5;
        let shift   = 2.0 * rad;
        let centerx = shift * (width as f32) / 2.0;

        for i in 0usize .. height {
            for j in 0usize .. width {
                let fj = j as f32;
                let fi = i as f32;
                let x  = fj * 2.0 * rad - centerx;
                let y  = -fi * 2.0 * rad - 0.04 - rad;

                let dynamic_block = Block::new(RigidBody::new_dynamic(Cuboid::new(Vector2::new(rad - 0.04, rad - 0.04)), 1.0, 0.3, 0.6), &mut s.physics_world);

                dynamic_block.body.borrow_mut().append_translation(&Vector2::new(x, y));
            }
        }

        Ok(s)
    }

    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        self.physics_world.step(timer::duration_to_f64(dt) as f32);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ctx.renderer.clear();
        graphics::draw(ctx, &mut self.text, None, None)?;
        let destrect = graphics::Rect::new(50, 50, 100, 100);
        graphics::set_color(ctx, graphics::Color::RGB(100, 20, 20));
        graphics::rectangle(ctx, graphics::DrawMode::Fill, destrect);
        graphics::set_color(ctx, graphics::Color::RGB(0, 0, 0));
        ctx.renderer.present();
        timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "ExploPlane".to_string();
    c.window_width = 1280;
    c.window_height = 720;

    let mut game: Game<MainState> = Game::new("helloworld", c).unwrap();
    if let Err(e) = game.run() {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
