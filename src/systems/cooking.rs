use amethyst::ecs::System
use crate::skill::Skill;

struct Cooking;

struct Food {
    index: u8,
    key: String,
    skill_list: Vec<Skill>
}

impl<'a> System<'a> for Cooking {
    type SystemData = (
        ReadExpect<'a, Resource>
        ReadStorage<'a, Component>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        println!("Hello!");
        // let mut food = Vec<Skill> = Vec::new();
    }
}