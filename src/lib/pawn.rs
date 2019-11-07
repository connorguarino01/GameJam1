enum Pawn {
    Cook,
    Server,
    Customer
}

pub struct Cook {
    name: String,
    mood: u8,
    speed: u8
}

pub struct Server {
    name: String,
    mood: u8,
    speed: u8
}

pub struct Customer {
    name: String,
    mood: u8,
    speed: u8
}

enum CookSkillTypes {
    Meat(u8),
    Vegetable(u8),
    Soup(u8),
    Baking(u8),
    Dessert(u8),
    Pasta(u8),
    Preserving(u8),
    Sandwich(u8),
}

enum ServerSkillTypes {
    Balance(u8),
    Hosting(u8),
    Social(u8),
}

trait SkillLevel {
    /// Given an XP Amount, add to total
    fn add_xp(&mut self, xp_gain:u8);
    /// Given an XP Amount, subtract from total
    fn sub_xp(&mut self, xp_loss:u8);
    /// When called, gives the value of the thing
    fn get_level(&self) -> u8;  
}

impl SkillLevel for CookSkillTypes {
    /// Adds an amount of xp to the specific variant of CookSkillTypes
    fn add_xp(&mut self, xp_gain:u8) {
        match self {
            CookSkillTypes::Meat(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Vegetable(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Soup(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Baking(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Dessert(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Pasta(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Preserving(ref mut cur_level) => *cur_level += xp_gain,
            CookSkillTypes::Sandwich(ref mut cur_level) => *cur_level += xp_gain,
        }
    }
    /// Subtracts an amount of xp to the specific variant of CookSkillTypes
    fn sub_xp(&mut self, xp_loss: u8) {
        match self {
            CookSkillTypes::Meat(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Vegetable(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Soup(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Baking(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Dessert(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Pasta(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Preserving(ref mut cur_level) => *cur_level -= xp_loss,
            CookSkillTypes::Sandwich(ref mut cur_level) => *cur_level -= xp_loss,
        }
    }
    /// Returns the value of the specific variant of CookSkillTypes
    fn get_level(&self) -> u8 {
        match self {
            CookSkillTypes::Meat(cur_level) => *cur_level,
            CookSkillTypes::Vegetable(cur_level) => *cur_level,
            CookSkillTypes::Soup(cur_level) => *cur_level,
            CookSkillTypes::Baking(cur_level) => *cur_level,
            CookSkillTypes::Dessert(cur_level) => *cur_level,
            CookSkillTypes::Pasta(cur_level) => *cur_level,
            CookSkillTypes::Preserving(cur_level) => *cur_level,
            CookSkillTypes::Sandwich(cur_level) => *cur_level
        }
    }
}

impl SkillLevel for ServerSkillTypes {
    /// Adds an amount of xp to the specific variant of ServerSkillTypes
    fn add_xp(&mut self, xp_gain:u8) {
        match self {
            ServerSkillTypes::Balance(ref mut cur_level) => *cur_level += xp_gain,
            ServerSkillTypes::Social(ref mut cur_level) => *cur_level += xp_gain,
            ServerSkillTypes::Hosting(ref mut cur_level) => *cur_level += xp_gain
        }
    }
    /// Subtracts an amount of xp to the specific variant of ServerSkillTypes
    fn sub_xp(&mut self, xp_loss:u8) {
        match self {
            ServerSkillTypes::Balance(ref mut cur_level) => *cur_level -= xp_loss,
            ServerSkillTypes::Social(ref mut cur_level) => *cur_level -= xp_loss,
            ServerSkillTypes::Hosting(ref mut cur_level) => *cur_level -= xp_loss
        }
    }
    /// Returns the value of the specific variant of ServerSkillTypes
    fn get_level(&self) -> u8 {
        match self {
            ServerSkillTypes::Balance(cur_level) => *cur_level,
            ServerSkillTypes::Social(cur_level) => *cur_level,
            ServerSkillTypes::Hosting(cur_level) => *cur_level 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn skill_level_test() {
        let mut cook_skill = CookSkillTypes::Meat(5);
        let mut server_skill = ServerSkillTypes::Balance(10);
        cook_skill.add_xp(10);
        server_skill.sub_xp(4);
        assert_eq!(cook_skill.get_level(), 15);
        assert_eq!(server_skill.get_level(), 6);
    }
}