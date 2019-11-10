use amethyst::ecs::{Component, VecStorage};

pub struct Skill {
    skill_type: SkillType,
    xp: u8,
    skill_level: u8,
}

pub enum SkillType {
    Meat,
    Vegetable
}

pub struct SkillList {
    skill_list: Vec<Skill>
}

impl SkillList {
    pub fn create_skill_list(skill_list: Vec<Skill>) -> SkillList {
        SkillList {
            skill_list,
        }
    }
}

impl Component for SkillList {
    type Storage = VecStorage<Self>;
}