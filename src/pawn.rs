use amethyst::{
    prelude::*,
    core::Transform,
    ecs::{ Entity, Component, NullStorage },
    renderer::{ sprite::SpriteSheetHandle, SpriteRender, Transparent }
};

#[derive(Default)]
pub struct Pawn;

impl Component for Pawn {
    type Storage = NullStorage<Self>;
}