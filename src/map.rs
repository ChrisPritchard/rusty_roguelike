use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y:i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Map {
        Self { tiles: vec![TileType::Floor; NUM_TILES] }
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.x < SCREEN_WIDTH && p.y >= 0 && p.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, p: Point) -> bool {
        self.in_bounds(p) && self.tiles[map_idx(p.x, p.y)] != TileType::Wall
    }

    pub fn try_idx(&self, p:Point) -> Option<usize> {
        if !self.in_bounds(p) {
            return None
        } 
        Some(map_idx(p.x, p.y))
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let pos = self.index_to_point2d(idx);
        let mut res = SmallVec::new();
        
        let left = Point::new(pos.x - 1, pos.y);
        if self.can_enter_tile(left) {
            res.push((self.point2d_to_index(left), 1.))
        }

        let up = Point::new(pos.x, pos.y - 1);
        if self.can_enter_tile(up) {
            res.push((self.point2d_to_index(up), 1.))
        }

        let right = Point::new(pos.x + 1, pos.y);
        if self.can_enter_tile(right) {
            res.push((self.point2d_to_index(right), 1.))
        }

        let down = Point::new(pos.x, pos.y + 1);
        if self.can_enter_tile(down) {
            res.push((self.point2d_to_index(down), 1.))
        }

        res
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point { x: SCREEN_WIDTH, y: SCREEN_HEIGHT }
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}