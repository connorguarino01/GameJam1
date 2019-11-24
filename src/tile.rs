use amethyst::{
    prelude::*,
    tiles::Tile,
    renderer::palette::rgb::Srgba,
    core::math::Point3
};

#[derive(Default, Clone)]
pub struct WorldTile;
impl Tile for WorldTile {
    fn sprite(&self, coordinates: Point3<u32>, _world: &World) -> Option<usize> {
        // Get X coordinate
        if coordinates.to_homogeneous()[0] % 2 == 0 {
            Some(0)
        } else {
            Some(1)
        }
    }
    fn tint(&self, _coordinates: Point3<u32>, _world: &World) -> Srgba {
        Srgba::new(1.0, 1.0, 1.0, 1.0)
    }
}