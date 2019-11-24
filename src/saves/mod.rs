//! This module contains all the necessary loaders and savers for the game's persistant data.
//! This includes, but not limited to, the tile's metadata, the pawns metadata, and map's
//! metadata.

/// This is the structured form of the file the game loads and saves to.
pub struct GameSave {
    tiles: Vec<Tile>,
    pawns: Vec<Pawn>,
    structures: Vec<Structure>,
    map: {
        height: u8
        width: u8
    }
}

