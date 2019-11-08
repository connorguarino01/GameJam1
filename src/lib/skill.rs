const XP_MAX:u8 = 100;
const XP_MIN:u8 = 0;
const SKILL_MAX:u8 = 10;

pub struct Skill {
    pub skilllevel: u8,
    pub xpamount: u8,
}

pub struct Cook {
    pub meat: Skill,
    pub vegetable: Skill,
    pub soup: Skill,
    pub baking: Skill,
    pub dessert: Skill,
    pub pasta: Skill,
    pub preserving: Skill,
    pub sandwich: Skill,
}

pub struct Server {
    pub balance: Skill,
    pub hosting: Skill,
    pub social: Skill,
}

pub enum CookSkills {
    Meat(u8),
    Vegetable(u8),
    Soup(u8),
    Baking(u8),
    Dessert(u8),
    Pasta(u8),
    Preserving(u8),
    Sandwich(u8),
}

impl Skill {
    pub fn xp_gain(&mut self, xp_gain:u8) {
        let temp_xpamount = self.xpamount + xp_gain;
        if temp_xpamount >= XP_MAX {
            self.xpamount = XP_MIN;
            self.skill_gain(1)
        } else {
            self.xpamount = temp_xpamount;
        }
    }
    pub fn skill_gain(&mut self, skill_gain:u8) {
        let temp_skillamount = self.skilllevel + skill_gain;
        if temp_skillamount > SKILL_MAX {
            self.skilllevel = SKILL_MAX;
        } else {
            self.skilllevel = temp_skillamount;
        }
    }
} 