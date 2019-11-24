use amethyst::{
    prelude::*,
    assets::{ Loader, AssetStorage, RonFormat },
    core::{ Transform, math::Vector3 },
    ecs::Entity,
    renderer::{
        debug_drawing::DebugLinesComponent,
        formats::texture::ImageFormat,
        sprite::{ SpriteSheet, SpriteSheetFormat, SpriteSheetHandle },
        Camera, Texture,
    },
    tiles::TileMap,
    window::ScreenDimensions,
};
use ron;

use crate::saves::GameSave;
use crate::pawn::Pawn;
use crate::foods::food::{Food, FoodHandle};

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Pawn>();
        // world.register::<Food>();

        // Load in sprites
        let pawn_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/pawns_spritesheet.png", "texture/pawns_spritesheet.ron");
        let map_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/map_spritesheet.png", "texture/map_spritesheet.ron");
        
        // Load in food
        // let _food_handle =
        //     load_food(world, "data/food.ron");
        
        let game_save = load_gamesave("data/save.ron"); // TODO: Figure out how to path to "<assets_dir> + data/save.ron"
        game_save.load_camera(world);
        game_save.load_map(world, map_sprite_sheet_handle.clone());
        game_save.load_pawns(world, pawn_sprite_sheet_handle);
        game_save.load_tile_map(world);
        game_save.load_structures(world, map_sprite_sheet_handle); // TODO: Think about structures sprite locations
    }

    // fn on_stop(_data: StateData<'_, GameData<'_, '_>>) {
    //     let world = data.world;
    // }

    // fn on_resume(_data: StateData<'_, GameData<'_, '_>>) {
    //     let world = data.world;
    // }

    // fn on_pause(_data: StateData<'_, GameData<'_, '_>>) {
    //     let world = data.world;
    // }
}

fn load_gamesave(ron_path: &str) -> GameSave {
    let gamesave_data = std::fs::read(ron_path).expect("The game save file could not be read!");
    ron::de::from_bytes(&gamesave_data).expect("The game save file is corrupt!")
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn load_food(world: &mut World, ron_path: &str) -> FoodHandle {
    let loader = world.read_resource::<Loader>();
    let food_store = world.read_resource::<AssetStorage<Food>>();
    loader.load(
        ron_path,
        RonFormat,
        (),
        &food_store,
    )
}