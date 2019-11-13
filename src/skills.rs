use amethyst::ecs::{Component, VecStorage};

pub struct Skill {
    xp: u8,
    skill_level: u8,
}

pub struct SkillList {
    meat: Option<Skill>,
    vegetable: Option<Skill>
}

impl Component for SkillList {
    type Storage = VecStorage<Self>;
}