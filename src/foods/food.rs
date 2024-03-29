use crate::foods::Skill;
use serde::{Deserialize, Serialize};
use amethyst::{
    assets::{Handle, Asset, ProcessingState},
    ecs::{ VecStorage, Component, DenseVecStorage },
    Error
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Component)]
pub struct Food {
    pub index: u8,
    pub key: String,
    pub skill_list: Vec<Skill>
}

pub type FoodHandle = Handle<Food>;

impl Asset for Food {
    const NAME: &'static str = "bin::food::Food";
    type Data = Food;
    type HandleStorage = VecStorage<FoodHandle>;
}

impl From<Food> for Result<ProcessingState<Food>, Error> {
    fn from(food: Food) -> Result<ProcessingState<Food>, Error> {
        match food {
            Food {index, key, skill_list} => {
                Ok(ProcessingState::Loaded(Food {
                    index,
                    key,
                    skill_list,
                }))
            }
        }
    }
}