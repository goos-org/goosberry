use crate::ecs::components::Component;
use crate::rendering::camera::Camera2d;
use downcast_rs::Downcast;
use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct Entity {
    components: Vec<Box<dyn Component>>,
}
impl Entity {
    pub fn add_component<T: Component>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.get_components().next()
    }
    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_components_mut().next()
    }
    pub fn get_components<T: 'static>(&self) -> impl Iterator<Item = &T> {
        self.components
            .iter()
            .filter_map(|c| c.deref().as_any().downcast_ref())
    }
    pub fn get_components_mut<T: 'static>(&mut self) -> impl Iterator<Item = &mut T> {
        self.components
            .iter_mut()
            .filter_map(|c| c.deref_mut().as_any_mut().downcast_mut())
    }
    pub(crate) fn filter_typeid(&self, typeid: TypeId) -> impl Iterator<Item = &(dyn Component)> {
        self.components.iter().filter_map(move |c| {
            if c.deref().as_any().type_id() == typeid {
                Some(c.deref())
            } else {
                None
            }
        })
    }
}
