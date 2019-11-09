use amethyst::ecs::{Component, VecStorage};

struct Skill {
    xp: u8,
    skill_level: u8,
}

enum SkillType {
    Meat(Skill),
    Vegetable(Skill),
    Soup(Skill),
    Baking(Skill),
    Dessert(Skill),
    Pasta(Skill),
    Preserving(Skill),
    Sandwich(Skill),
}

impl Component for SkillType {
    type Storage = VecStorage<Self>;
}