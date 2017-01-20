use ggez::graphics;
use ggez::Context;

use na::Vector2;
use ncollide::shape::Cuboid;

use block::Block;


pub fn draw_rectangle(ctx: &mut Context, block: &Block) {
    let body = block.body.borrow();

    if body.shape().is_shape::<Cuboid<Vector2<f32>>>() {
        let shape: &Cuboid<Vector2<f32>> = body.shape().as_shape().unwrap();
        let extents = shape.half_extents();
        let destrect = graphics::Rect::new(
            body.position().translation.x as i32,
            body.position().translation.y as i32,
            extents.x as u32,
            extents.y as u32,
        );
        let _ = graphics::rectangle(ctx, graphics::DrawMode::Fill, destrect);
    }
}
