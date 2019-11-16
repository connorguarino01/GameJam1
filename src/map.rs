///! Loads and saves data for the "save" of the game to a `.ron` file located in the `assets/saves` directory

pub struct MapData {
    pawns: Vec<Pawn>,
    tiles: Vec<TileType>,
}