use nphysics2d::object::{RigidBody, RigidBodyHandle};
use nphysics2d::world::World;

pub struct Block {
    pub body: RigidBodyHandle<f32>,
}

impl Block {
    pub fn new(body: RigidBody<f32>, physics_world: &mut World<f32>) -> Block {
        let rb_handle = physics_world.add_rigid_body(body);
        let b = Block { body: rb_handle };
        b
    }
}
