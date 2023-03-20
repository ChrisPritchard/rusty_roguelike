use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mb = MapBuilder::blank();
        mb
    }
}