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

pub fn initialize_pawns(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(Pawn)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}