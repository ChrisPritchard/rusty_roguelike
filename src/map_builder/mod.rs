use crate::prelude::*;

use self::empty::EmptyArchitect;
const NUM_ROOMS: usize = 20;

mod empty;

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

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH-11), 
                rng.range(1, SCREEN_HEIGHT-11), 
                rng.range(2, 10), 
                rng.range(2, 10));
            let mut overlaps = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlaps = true;
                    break;
                }
            }
            if overlaps {
                continue;
            }
            room.for_each(|p| self.map.tiles[map_idx(p.x, p.y)] = TileType::Floor);
            self.rooms.push(room);
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let architect = EmptyArchitect {};
        architect.new(rng)
        // let mut mb = MapBuilder{
        //     map: Map::new(),
        //     rooms: Vec::new(),
        //     player_start: Point::zero(),
        //     amulet_start: Point::zero(),
        //     monster_spawns: Vec::new(),
        // };
        // mb.fill(TileType::Wall);
        // mb.build_random_rooms(rng);
        // mb.build_corridors(rng);
 
        // mb.player_start = mb.rooms[0].center();
        // let furthest = mb.find_most_distant();
        // mb.amulet_start = furthest;

        // for r in mb.rooms.iter().skip(1) {
        //     mb.monster_spawns.push(r.center());
        // }

        // mb
    }
}