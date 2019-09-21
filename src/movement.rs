use std::marker::PhantomData;
use amethyst::{
	ecs::prelude::*,
	core::transform::Transform,
};
use nalgebra_glm::*;
use crate::{
	voxel::AsVoxel,
	world::MutableVoxelWorld,
	collision::*,
};

pub struct Pos { 
	pub position: Vec3, 
	pub velocity: Vec3,
}

pub struct MovementSystem<V: AsVoxel> {
	marker: PhantomData<V>,
}

impl Component for Pos {
	type Storage = DenseVecStorage<Self>;
}

impl<V: AsVoxel> MovementSystem<V> {
	pub fn new() -> Self {
		Self {
			marker: PhantomData,
		}
	}
}

impl<'a, V: 'static + AsVoxel> System<'a> for MovementSystem<V> where V::Voxel: Raycast {
	type SystemData = (
		ReadStorage<'a, MutableVoxelWorld<V>>,
		WriteStorage<'a, Pos>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (worlds, mut positions, mut transforms): Self::SystemData) {
		for (pos, transform) in (&mut positions, &mut transforms).join() {
			let velocity = (&worlds).join().fold(pos.velocity, |mut velocity, world| {
				for i in 0..3 {
					let mut dir = vec3(0.0, 0.0, 0.0);
					dir[i] = velocity[i];
					velocity[i] *= world
						.hit(&world.ray(pos.position, dir))
						.unwrap_or(1.0)
						.min(1.0);
				}
				velocity
			});

			pos.position += velocity;

			transform.set_translation(pos.position);
		}
	}
}
