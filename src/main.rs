pub mod utils;
pub mod block;

extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics2d;
extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::game::{Game, GameState};
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;

use std::time::Duration;

use rand::Rand;

use na::Vector2;
use ncollide::shape::{Plane, Cuboid};
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;

use block::Block;
use utils::draw_rectangle;



struct MainState {
    text: graphics::Text,
    physics_world: World<f32>,
    blocks: Vec<Block>,
    player: Block,
}


impl GameState for MainState {
    fn load(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "DejaVuSerif.ttf", 48).unwrap();
        let text = graphics::Text::new(ctx, "Hello world!", &font).unwrap();

        let mut world = World::new();
        let explane = Block::new(RigidBody::new_dynamic(Cuboid::new(Vector2::new(10.0, 100.0)), 1.0, 0.3, 0.6), &mut world);

        let mut s = MainState {
            text: text,
            physics_world: world,
            blocks: Vec::new(),
            player: explane,
        };

        s.physics_world.set_gravity(Vector2::new(0.0, 9.81));

        let static_block = Block::new(RigidBody::new_static(Plane::new(Vector2::new(0.0, -1.0)), 0.3, 0.6), &mut s.physics_world);
        static_block.body.borrow_mut().append_translation(&Vector2::new(0.0, 500.0));
        s.blocks.push(static_block);

        s.player.body.borrow_mut().append_translation(&Vector2::new(100.0, 100.0));

        Ok(s)
    }

    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        self.physics_world.step(timer::duration_to_f64(dt) as f32);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();

        ctx.renderer.clear();
        graphics::draw(ctx, &mut self.text, None, None)?;
        for block in &self.blocks {
            graphics::set_color(ctx, graphics::Color::rand(&mut rng));
            draw_rectangle(ctx, block);
        }

        graphics::set_color(ctx, graphics::Color::RGB(0, 1, 0));
        draw_rectangle(ctx, &self.player);

        graphics::set_color(ctx, graphics::Color::RGB(0, 0, 0));
        ctx.renderer.present();
        timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "ExPlane".to_string();
    c.window_width = 1280;
    c.window_height = 720;

    let mut game: Game<MainState> = Game::new("helloworld", c).unwrap();
    if let Err(e) = game.run() {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
