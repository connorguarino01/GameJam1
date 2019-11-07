enum Pawn {
    Worker,
    Customer
}

pub struct Worker {
    name: String;
    mood: u32;
    speed: u32;
    skills: Skills;
}

pub struct Customer {
    pub name: String;
    pub mood: u32;
    speed: u32;
}

pub struct Skills {
    meat: u32;
    vegetable: u32;
    soup: u32;
    baking: u32;
    dessert: u32;
    pasta: u32;
    preserving: u32;
    sandwich: u32;
    hosting: u32;
}

pub trait Skill_Bounds {
    fn skill_bounds(&self) {

    }
}

impl Skill_Bounds for Skills {

}