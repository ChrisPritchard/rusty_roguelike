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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    match self.tiles[map_idx(x, y)] {
                        TileType::Floor => ctx.set(x - camera.left_x, y - camera.top_y, YELLOW, BLACK, to_cp437('.')),
                        TileType::Wall => ctx.set(x - camera.left_x, y - camera.top_y, GREEN, BLACK, to_cp437('#')),
                    }
                }
            }
        }
    }

    fn in_bounds(&self, p: Point) -> bool {
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