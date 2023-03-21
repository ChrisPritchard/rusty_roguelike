use crate::prelude::*;

use self::drunkards_walk::DrunkardsWalkArchitect;

const NUM_MONSTERS: usize = 50;

mod empty;
mod rooms;
mod cellular_automata;
mod drunkards_walk;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub monster_spawns: Vec<Point>,
}

impl MapBuilder {
    fn blank() -> Self {
        Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            monster_spawns: Vec::new(),
        }
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&mut self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH, SCREEN_HEIGHT, 
            &vec![self.map.point2d_to_index(self.player_start)], 
            &self.map, 
            1024.);
        let index = 
            dijkstra_map.map.iter().enumerate()
            .filter(|(_, dist)| **dist < f32::MAX)
            .max_by(|a, b| a.1.partial_cmp(b.1)
            .unwrap()).unwrap().0;
        self.map.index_to_point2d(index)
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let mut spawnable_tiles = Vec::new();
        for idx in 0..self.map.tiles.len() {
            if self.map.tiles[idx] != TileType::Floor {
                continue;
            }
            let p = self.map.index_to_point2d(idx);
            let dist = DistanceAlg::Pythagoras.distance2d(*start, p);
            if dist > 10. {
                spawnable_tiles.push(p);
            }
        }
        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_idx = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_idx].clone());
            spawnable_tiles.remove(target_idx);
        }
        spawns
    }

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect = DrunkardsWalkArchitect {};
        architect.new(rng)
    }
}