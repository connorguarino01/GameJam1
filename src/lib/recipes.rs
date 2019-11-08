use crate::skill::{CookSkills};

pub struct Food {
    pub req_skills: Vec<CookSkills>,
    pub xp_award: u8,
}

pub enum Recipes {    
    Hamburger(Food),
    Steak(Food),
}

