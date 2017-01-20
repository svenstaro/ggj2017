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
use nphysics2d::object::RigidBody;

// First we make a structure to contain the game's state
struct MainState {
    text: graphics::Text,
    physics_world: World<f32>,
}


// Then we implement the `ggez::game::GameState` trait on it, which
// requires callbacks for creating the game state, updating it each
// frame, and drawing it.
//
// The `GameState` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl GameState for MainState {
    fn load(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "DejaVuSerif.ttf", 48).unwrap();
        let text = graphics::Text::new(ctx, "Hello world!", &font).unwrap();

        let s = MainState { text: text, physics_world: World::new() };

        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 9.81));

        let rb = RigidBody::new_static(Plane::new(Vector2::new(0.0, -1.0)), 0.3, 0.6);
        world.add_rigid_body(rb);

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

                let mut rb = RigidBody::new_dynamic(Cuboid::new(Vector2::new(rad - 0.04, rad - 0.04)), 1.0, 0.3, 0.6);

                rb.append_translation(&Vector2::new(x, y));

                world.add_rigid_body(rb);
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

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title,
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game,
// * then just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let c = conf::Conf::new();
    let mut game: Game<MainState> = Game::new("helloworld", c).unwrap();
    if let Err(e) = game.run() {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
