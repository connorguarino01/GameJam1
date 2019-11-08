use crate::skill::{CookSkills, Cook, Server};
use crate::recipes::{Recipes};

impl Recipes {
    fn get_reqskills(&self) -> Vec<CookSkills> {
        match self {
            Recipes::Hamburger(recipe) => recipe.req_skills,
            Recipes::Steak(recipe) => recipe.req_skills,
        }
    }
}

pub trait Cooking {
    /// Determines of outcome of Pawn cooking meat
    fn cook(&mut self, _food:Recipes) -> bool {
        false
    }
}

impl Cooking for Cook {
    fn cook(&mut self, food:Recipes) -> bool {
        let req_skills = food.get_reqskills();
        let mut recipe_check = true;
        for cookingskill in req_skills.iter() {
            let skill_check = match cookingskill {
                CookSkills::Meat(req_skill) => self.meat.skilllevel >= *req_skill,
            };
            if !skill_check {
                recipe_check = false; 
                break; 
            }
        };
        if recipe_check {
            for cookingskill in req_skills.iter() {
                match cookingskill {
                    CookSkills::Meat(_) => self.meat.xp_gain(xp_award)
                }
            }
        }
        recipe_check
    }
}

impl Cooking for Server {
}