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

use crate::pawn::{ Pawn, initialize_pawns };
use crate::tile::WorldTile;
use crate::bin::food::{Food, FoodHandle};

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Pawn>();

        // Load in sprites
        let pawn_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/pawns_spritesheet.png", "texture/pawns_spritesheet.ron");
        let map_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/map_spritesheet.png", "texture/map_spritesheet.ron");
        
        // Load in food
        let food_handle=
            load_food(world, "data/food.ron");

        // Setup camera
        // let (width, height) = {
        //     let dim = world.read_resource::<ScreenDimensions>();
        //     (dim.width(), dim.height())
        // };
        
        let _player = initialize_pawns(world, &pawn_sprite_sheet_handle);

        let width = 128.0 * 20.0;
        let height = 128.0 * 20.0;
        let _camera = initialise_camera(
            world,
            Transform::from(Vector3::new(0.0, 0.0, 1.1)),
            Camera::standard_2d(width, height),
        );

         // Create a test debug lines entity
         let _ = world
            .create_entity()
            .with(DebugLinesComponent::with_capacity(1))
            .build();

        let map = TileMap::<WorldTile>::new(
            Vector3::new(10, 10, 1),
            Vector3::new(128, 128, 1),
            Some(map_sprite_sheet_handle),
        );

        let _map_entity = world
            .create_entity()
            .with(map)
            .with(Transform::default())
            .build();
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

fn initialise_camera(world: &mut World, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(camera)
        .with(transform)
        .named("camera")
        .build()
}