use goosberry::ecs::components::Transform3;
use goosberry::ecs::entity::Entity;
use goosberry::ecs::game::Game;
use goosberry::ecs::world::World;
use goosberry::rendering::camera::Camera2d;
use goosberry::rendering::render_2d;
use nalgebra::Vector2;
use std::time::Duration;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, WindowBuilder};

pub fn example_system(world: &World) {
    for mut entity in world.query_mut::<(Transform3<f32>,)>() {
        if let Some(transform) = entity.get_components_mut::<Transform3<f32>>().next() {
            transform.position.x += 1.0;
        }
    }
}

#[tokio::main]
async fn main() {
    let wb = WindowBuilder::new()
        .with_title("Goosberry example")
        .with_fullscreen(Some(Fullscreen::Borderless(None)));
    let event_loop = EventLoop::new();
    let window = wb.build(&event_loop).unwrap();

    let mut entity = Entity::default();
    entity.add_component(Transform3::<f32>::default());

    let mut camera = Entity::default();
    camera.add_component(
        Camera2d::new(
            &window,
            Vector2::new(window.inner_size().width, window.inner_size().height),
        )
        .await,
    );

    let mut world = World::default();
    world.add_entity(entity);
    world.add_entity(camera);

    let mut game = Game::new(world);
    game.add_system(example_system);
    game.add_system(render_2d);

    let mut total = Duration::new(0, 0);
    let mut frames = 0;

    event_loop.run(move |e, target, control_flow| {
        control_flow.set_poll();
        match e {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let mut entity = game.world.query_mut::<(Camera2d,)>().next().unwrap();
                let camera = entity.get_component_mut::<Camera2d>().unwrap();
                camera.resize(Vector2::new(size.width, size.height));
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                game.update();
                total += game.delta_time;
                frames += 1;
                print!(
                    "\rAvg. Framerate: {}",
                    1.0 / (total.as_secs_f32() / frames as f32)
                );
            }
            _ => {}
        }
    });
}
