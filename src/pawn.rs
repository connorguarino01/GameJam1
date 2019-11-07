enum Pawn {
    Cook,
    Server,
    Customer
}

pub struct Cook {
    name: String;
    mood: u8;
    speed: u8;
}

pub struct Server {
    name: String;
    mood: u8;
    speed: u8;
}

pub struct Customer {
    name: String;
    mood: u8;
    speed: u8;
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
    pub fn add_xp(&mut self, xp_gain:u8);
    /// Given an XP Amount, subtract from total
    pub fn sub_xp(&mut self, xp_loss:u8);
    /// When called, gives the value of the thing
    pub fn get_level(&self) -> u8;  
}

impl SkillLevel for CookSkillTypes {
    /// Adds an amount of xp to the specific variant of CookSkillTypes
    fn add_xp(&mut self, xp_gain:u8) {
        self.0 = self.0 + xp_gain;
    }
    /// Subtracts an amount of xp to the specific variant of CookSkillTypes
    fn sub_xp(&mut self, xp_loss:u8) {
        self.0 = self.0 - xp_loss;
    }
    /// Returns the value of the specific variant of CookSkillTypes
    fn get_level(&self) -> u8 {
        &self.0
    }
}

impl SkillLevel for ServerSkillTypes {
    /// Adds an amount of xp to the specific variant of ServerSkillTypes
    fn add_xp(&mut self, xp_gain:u8) {
        self.0 = self.0 + xp_gain;
    }
    /// Subtracts an amount of xp to the specific variant of ServerSkillTypes
    fn sub_xp(&mut self, xp_loss:u8) {
        self.0 = self.0 - xp_loss;
    }
    /// Returns the value of the specific variant of ServerSkillTypes
    fn get_level(&self) -> u8 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn skill_level_test() {
        let cook_skill = CookSkillTypes::Meat(5);
        let server_skill = ServerSkillTypes::Balance(10);
        cook_skill.add_xp(10);
        server_skill.sub_xp(14);
        assert_eq!(cook_skill.get_level(), 15);
        assert_eq!(server_skill.get_level(), 6);
    }
}