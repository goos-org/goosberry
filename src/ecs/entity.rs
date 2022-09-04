use std::any::{Any, TypeId};
use std::ops::Deref;

#[derive(Default)]
pub struct Entity {
    components: Vec<Box<dyn Any + Send + Sync>>,
}
impl Entity {
    pub fn add_component<T: 'static + Any + Send + Sync>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.get_components().next()
    }
    pub fn get_components<T: 'static>(&self) -> impl Iterator<Item = &T> {
        self.components.iter().filter_map(|c| c.downcast_ref())
    }
    pub fn get_components_mut<T: 'static>(&mut self) -> impl Iterator<Item = &mut T> {
        self.components.iter_mut().filter_map(|c| c.downcast_mut())
    }
    pub(crate) fn filter_typeid(
        &self,
        typeid: TypeId,
    ) -> impl Iterator<Item = &(dyn Any + Send + Sync)> {
        self.components.iter().filter_map(move |c| {
            if c.deref().type_id() == typeid {
                Some(c.deref())
            } else {
                None
            }
        })
    }
}
