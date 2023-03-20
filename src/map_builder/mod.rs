use crate::prelude::*;

use self::rooms::RoomsArchitect;

mod empty;
mod rooms;
mod cellular_automata;

trait MapArchitect {
    fn new(&self, rng: &mut RandomNumberGenerator) -> MapBuilder;
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

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let architect = RoomsArchitect {};
        architect.new(rng)
    }
}