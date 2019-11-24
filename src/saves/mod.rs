//! This module contains all the necessary loaders and savers for the game's persistant data.
//! This includes, but not limited to, the tile's metadata, the pawns metadata, and map's
//! metadata.
use amethyst::{
    prelude::*,
    assets::Handle,
    tiles::TileMap,
    core::{ Transform, math::Vector3 },
    renderer::{
        debug_drawing::DebugLinesComponent,
        sprite::{ SpriteSheet, SpriteRender },
        Camera, Transparent
    }
};
use serde::{ Serialize, Deserialize };
use crate::tile::WorldTile;
use crate::pawn::Pawn;

#[derive(Serialize, Deserialize)]
pub struct IntermediateTile {
    postion: [u8; 3],
    id: u8
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IntermediatePawn {
    position: [f32; 3],
    job: String,
    name: String,
    skills: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct IntermediateStructure {

}

#[derive(Serialize, Deserialize)]
pub struct IntermediateMap {
    map_height: u32,
    map_width: u32,
    tile_height: u32,
    tile_width: u32
}

/// This is the structured form of the file the game loads and saves to.
#[derive(Serialize, Deserialize)]
pub struct GameSave {
    tiles: Vec<IntermediateTile>,
    pawns: Vec<IntermediatePawn>,
    structures: Vec<IntermediateStructure>,
    map: IntermediateMap
}

impl GameSave {
    // Loads in the structures as entities
    pub fn load_structures(&self, world: &mut World, structures_sprite_sheet_handle: Handle<SpriteSheet>) {
        unimplemented!();
    }

    // Loads in tile metadata as an assets for future use in crate::tile::WorldTile
    pub fn load_tile_map(&self, world: &mut World) {
        unimplemented!();
    }

    // Loads in pawns as entities
    pub fn load_pawns(&self, world: &mut World, pawn_sprite_sheet_handle: Handle<SpriteSheet>) {
        for pawn in self.pawns.clone() { //TODO: better memory stuff
            let mut transform = Transform::default();
            transform.set_translation_xyz(pawn.position[0], pawn.position[1], pawn.position[2]); // z should be staticly set
            let sprite = SpriteRender {
                sprite_sheet: pawn_sprite_sheet_handle.clone(),
                sprite_number: 0,
            };
            world
                .create_entity()
                .with(transform)
                .with(Pawn)
                .with(sprite)
                .with(Transparent)
                .named(pawn.name)
                .build();
        }
    }

    // Creates the world map entity
    pub fn load_map(&self, world: &mut World, map_sprite_sheet_handle: Handle<SpriteSheet>) {
        // Create a test debug lines entity
        let _ = world
            .create_entity()
            .with(DebugLinesComponent::with_capacity(1))
            .build();
        let map = TileMap::<WorldTile>::new(
            Vector3::new(self.map.map_width, self.map.map_height, 1),
            Vector3::new(self.map.tile_width, self.map.tile_height, 1),
            Some(map_sprite_sheet_handle),
        );

        let _map_entity = world
            .create_entity()
            .with(map)
            .with(Transform::default())
            .build();
    }

    pub fn load_camera(&self, world: &mut World) {
        // Setup camera
        // let (width, height) = {
        //     let dim = world.read_resource::<ScreenDimensions>();
        //     (dim.width(), dim.height())
        // };
        let width = 128.0 * 20.0;
        let height = 128.0 * 20.0;
        world
            .create_entity()
            .with(Camera::standard_2d(width, height))
            .with(Transform::from(Vector3::new(0.0, 0.0, 1.1)))
            .named("camera")
            .build();
    }
}

