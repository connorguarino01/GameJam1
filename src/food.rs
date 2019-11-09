const SKILL_TYPES: [&str;2] = ["Meat", "Vegetable"];

struct Skill {
    skill_type: String,
    xp: u8,
    skill_level: u8,
}

struct Food {
    food_name: String,
    skills: Vec<Skill>,
}

impl Food {
    fn create_food(food_name: String, skills: Vec<Skill>) -> Food {
    }
}