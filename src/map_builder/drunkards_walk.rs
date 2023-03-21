use crate::prelude::*;
use super::MapArchitect;

const STAGGER_DISTANCE : usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect { }

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE || !map.in_bounds(drunkard_pos) {
                break;
            }
        }
    }

    fn desired_floor_reached(&self, map: &Map) -> bool {
        map.tiles.iter().filter(|t| **t == TileType::Floor).count() >= DESIRED_FLOOR
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::blank();
        mb.fill(TileType::Wall);

        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let center_idx = mb.map.point2d_to_index(center);
        self.drunkard(&center, rng, &mut mb.map);

        while !self.desired_floor_reached(&mb.map) {
            let point = Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT));
            self.drunkard(&point, rng, &mut mb.map);

            let dijkstra = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &vec![center_idx], &mb.map, 1024.);
            // any tile with the max f32 value wasn't reachable from the center
            // so make it a wall to close it off
            dijkstra.map.iter().enumerate().for_each(|(idx, dist)| {
                if *dist == f32::MAX {  
                    mb.map.tiles[idx] = TileType::Wall;
                }
            });
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}