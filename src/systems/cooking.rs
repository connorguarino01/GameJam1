use amethyst::ecs::System

struct Cooking

impl<'a> System<'a> for Cooking {
    type SystemData = (
        ReadExpect<'a, Resource>
        ReadStorage<'a, Component>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        println!("Hello!");
    }
}