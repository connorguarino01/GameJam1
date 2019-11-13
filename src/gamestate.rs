use amethyst::{
    prelude::*,
    assets::{ Loader, AssetStorage },
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

use crate::tile::WorldTile;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load in sprites
        let map_sprite_sheet_handle =
            load_sprite_sheet(world, "texture/cp437_20x20.png", "texture/cp437_20x20.ron");

        // Setup camera
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

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
            Vector3::new(48, 48, 1),
            Vector3::new(20, 20, 1),
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

fn initialise_camera(world: &mut World, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(camera)
        .with(transform)
        .named("camera")
        .build()
}