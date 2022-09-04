use crate::ecs::world::World;
use crate::ecs::System;
use std::time::Duration;

pub struct Game {
    pub world: World,
    pub delta_time: Duration,
    systems: Vec<Box<System>>,
    last_end: std::time::Instant,
}
impl Game {
    pub fn new(world: World) -> Self {
        Game {
            world,
            delta_time: Duration::new(0, 0),
            systems: Vec::new(),
            last_end: std::time::Instant::now(),
        }
    }
    pub fn add_system<F: 'static + FnMut(&World)>(&mut self, system: F) {
        self.systems.push(Box::new(system));
    }
    pub fn update(&mut self) {
        for system in &mut self.systems {
            system(&self.world);
        }
        self.delta_time = self.last_end.elapsed();
        self.last_end = std::time::Instant::now();
    }
}
