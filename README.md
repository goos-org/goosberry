# goosberry
A simple game engine with a unique design.

## Entities, Components, and Systems
You may already be familiar with the ECS paradigm, but if not, here's a quick rundown:

**Entities** are the objects in your game.
They can be anything from a player to a tree to a bullet.

**Components** are data attached to entities.
They can be anything from a position to a health value to a sprite.

**Systems** are logic that operates on entities.
They can be anything from a physics system to a rendering system to your game scripts.

goosberry uses this paradigm for everything in your game.
It is the heart of every piece of the engine.

## No window?
goosberry is a headless game engine.
This means that it does not provide a windowing system, or a built-in `main` function.
In order to run goosberry yourself, you'll need to provide your own `main` function.
This can be done relatively easily, like so
(using winit, but anything implementing `raw_window_handle::RawWindowHandle` can be used):
```rust
# struct Foo {
#     x: i32,
# }
# struct Bar {
#     x: i32,
# }
#
# fn some_system(world: &World) {
#     for mut entity in world.query_mut::<(Foo, Bar)>() {
#         if let Some(foo) = entity.get_components_mut::<Foo>().next() {
#             foo.x += 1;
#         }
#         if let Some(bar) = entity.get_components_mut::<Bar>().next() {
#             bar.x += 1;
#         }
#     }
# }
#
fn main() {
    let foo = Foo { x: 0 };
    let bar = Bar { x: 0 };
    let mut entity = Entity::default();
    entity.add_component(foo);
    entity.add_component(bar);
    let mut world = World::default();
    world.add_entity(entity);
    let mut game = Game::new(world);
    game.add_system(some_system);
    loop {
        game.update();
    }
}
```