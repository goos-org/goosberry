use crate::ecs::world::World;
use crate::rendering::camera::Camera2d;

pub mod camera;
mod render;
pub mod sprite;
pub mod texture;

pub fn render_2d(world: &World) {
    let mut entity = world.query_mut::<(Camera2d,)>().next().unwrap();
    let camera = entity.get_component_mut::<Camera2d>().unwrap();
    camera.render().expect("Rendering error");
}
