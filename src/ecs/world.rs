use crate::ecs::entity::Entity;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tuple_unpack::TupleUnpack;

/// ## Usage
/// ```rust
/// # use thing_box::ecs::entity::Entity;
/// # use thing_box::ecs::game::Game;
/// # use thing_box::ecs::world::World;
/// let foo = Foo { x: 0 };
/// let bar = Bar { x: 0 };
///
/// let mut entity = Entity::default();
/// entity.add_component(foo);
/// entity.add_component(bar);
///
/// let mut world = World::default();
/// world.add_entity(entity);
///
/// let mut game = Game::new(world);
/// game.add_system(some_system);
/// ```
#[derive(Default)]
pub struct World {
    pub(crate) entities: Vec<RwLock<Entity>>,
}
impl World {
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(RwLock::new(entity));
    }
    pub fn query<T: 'static + TupleUnpack>(&self) -> impl Iterator<Item = RwLockReadGuard<Entity>> {
        self.entities.iter().filter_map(|e| {
            let read = e.read().unwrap();
            let types = T::unpack_types();
            if types
                .iter()
                .any(|t| read.filter_typeid(*t).next().is_some())
            {
                Some(read)
            } else {
                None
            }
        })
    }
    pub fn query_mut<T: 'static + TupleUnpack>(
        &self,
    ) -> impl Iterator<Item = RwLockWriteGuard<Entity>> {
        self.entities.iter().filter_map(|e| {
            let read = e.read().unwrap();
            let types = T::unpack_types();
            if types
                .iter()
                .any(|t| read.filter_typeid(*t).next().is_some())
            {
                drop(read);
                Some(e.write().unwrap())
            } else {
                None
            }
        })
    }
}
