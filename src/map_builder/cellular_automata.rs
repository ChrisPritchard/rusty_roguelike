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

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> i32 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let p = Point::new(x+dx, y+dy);
                if map.in_bounds(p) && map.tiles[map.point2d_to_index(p)] == TileType::Wall {
                    count += 1;
                }
            }
        }
        count
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT-1 {
            for x in 1..SCREEN_WIDTH-1 {
                let neighbours = self.count_neighbours(x, y, map);
                let idx = map_idx(x, y);
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let mut closest_idx = 0;
        let mut closest_dist = f32::MAX;
        for idx in 0..map.tiles.len() {
            let point = map.index_to_point2d(idx);
            let dist = DistanceAlg::Pythagoras.distance2d(center, point);
            if dist < closest_dist {
                closest_idx = idx;
                closest_dist = dist;
            }
        }
        map.index_to_point2d(closest_idx)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mb = MapBuilder::blank();
        mb
    }
}