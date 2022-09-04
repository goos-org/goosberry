use crate::ecs::world::World;

pub mod components;
pub mod entity;
pub mod game;
pub mod world;

type System = dyn FnMut(&World);

#[cfg(test)]
mod tests {
    use crate::ecs::entity::Entity;
    use crate::ecs::game::Game;
    use crate::ecs::world::World;
    use std::time::Duration;

    struct Foo {
        x: i32,
    }
    struct Bar {
        x: i32,
    }

    fn some_system(world: &World) {
        for mut entity in world.query_mut::<(Foo, Bar)>() {
            if let Some(foo) = entity.get_components_mut::<Foo>().next() {
                foo.x += 1;
            }
            if let Some(bar) = entity.get_components_mut::<Bar>().next() {
                bar.x += 1;
            }
        }
    }

    #[test]
    fn test_ecs() {
        let foo = Foo { x: 0 };
        let bar = Bar { x: 0 };
        let mut entity = Entity::default();
        entity.add_component(foo);
        entity.add_component(bar);
        let mut world = World::default();
        world.add_entity(entity);
        let mut game = Game::new(world);
        game.add_system(some_system);
        let mut total_time = Duration::new(0, 0);
        let mut frames = 0;
        let mut lowest = Duration::new(0, 0);
        let mut highest = Duration::new(u64::MAX, 0);
        loop {
            game.update();
            total_time += game.delta_time;
            frames += 1;
            if game.delta_time > lowest {
                lowest = game.delta_time;
            }
            if game.delta_time < highest {
                highest = game.delta_time;
            }
            if total_time > Duration::new(5, 0) {
                break;
            }
        }
        println!(
            "Average: {}, lowest: {}, highest: {}, elapsed: {:?}",
            1_000_000_000 / (total_time.as_nanos() / frames),
            1_000_000_000 / lowest.as_nanos(),
            1_000_000_000 / highest.as_nanos(),
            total_time,
        );
    }
}
