const XP_MAX:u8 = 100;
const XP_MIN:u8 = 0;
const SKILL_MAX:u8 = 10;

pub struct Skill {
    skilllevel: u8,
    xpamount: u8,
}

impl Skill {
    fn xp_gain(&mut self, xp_gain:u8) {
        let temp_xpamount = self.xpamount + xp_gain;
        if temp_xpamount >= XP_MAX {
            self.xpamount = XP_MIN;
            self.skill_gain(1)
        } else {
            self.xpamount = temp_xpamount;
        }
    }
    fn skill_gain(&mut self, skill_gain:u8) {
        let temp_skillamount = self.skilllevel + skill_gain;
        if temp_skillamount > SKILL_MAX {
            self.skilllevel = SKILL_MAX;
        } else {
            self.skilllevel = temp_skillamount;
        }
    }
} 

pub struct Cook {
    meat: Skill,
}

pub struct Server {
    hosting: Skill,
}

pub struct Food {
    req_skills: Vec<CookSkills>,
    xp_award: u8,
}

pub enum Recipes {    
    Hamburger(Food),
    Steak(Food),
}

impl Recipes {
    fn get_reqskills(&self) -> Vec<CookSkills> {
        match self {
            Recipes::Hamburger(recipe) => recipe.req_skills,
            Recipes::Steak(recipe) => recipe.req_skills,
        }
    }
}

pub enum CookSkills {
    Meat(u8),
}

pub trait Cooking {
    /// Determines of outcome of Pawn cooking meat
    fn cook(&mut self, _food:Recipes) -> bool {
        false
    }
}

impl Cooking for Cook {
    fn cook(&mut self, food:Recipes) -> bool {
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