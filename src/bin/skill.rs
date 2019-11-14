use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Skill {
    skill_type: String,
    level: u8,
    xp: u8,
}