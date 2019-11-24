///! Loads and saves data for the "save" of the game to a `.ron` file located in the `assets/saves` directory
use amethyst::{
    prelude::*,
    assets::{Handle, Asset, ProcessingState},
    ecs::VecStorage,
    Error
};
use crate::pawn::Pawn;
use crate::tile::WorldTile;

pub struct MapData {
    pawns: Vec<Pawn>,
    tiles: Vec<WorldTile>,
}

pub type MapDataHandle = Handle<MapData>;

fn load_map_data(world: &mut World, ron_path: &str) -> MapDataHandle {

}