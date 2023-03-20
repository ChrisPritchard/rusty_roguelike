use crate::prelude::*;

const NUM_ROOMS: usize = 20;

use super::MapArchitect;

pub struct RoomsArchitect {}

fn build_random_rooms(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    while mb.rooms.len() < NUM_ROOMS {
        let room = Rect::with_size(
            rng.range(1, SCREEN_WIDTH-11), 
            rng.range(1, SCREEN_HEIGHT-11), 
            rng.range(2, 10), 
            rng.range(2, 10));
        let mut overlaps = false;
        for r in mb.rooms.iter() {
            if r.intersect(&room) {
                overlaps = true;
                break;
            }
        }
        if overlaps {
            continue;
        }
        room.for_each(|p| mb.map.tiles[map_idx(p.x, p.y)] = TileType::Floor);
        mb.rooms.push(room);
    }
}

fn apply_vertical_tunnel(mb: &mut MapBuilder, y1: i32, y2: i32, x: i32) {
    use std::cmp::{min, max};
    for y in min(y1, y2) ..= max(y1, y2) {
        if let Some(idx) = mb.map.try_idx(Point::new(x, y)) {
            mb.map.tiles[idx] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(mb: &mut MapBuilder, x1: i32, x2: i32, y: i32) {
    use std::cmp::{min, max};
    for x in min(x1, x2) ..= max(x1, x2) {
        if let Some(idx) = mb.map.try_idx(Point::new(x, y)) {
            mb.map.tiles[idx] = TileType::Floor;
        }
    }
}

fn build_corridors(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut rooms = mb.rooms.clone();
    rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

    for (i, room) in rooms.iter().enumerate().skip(1) {
        let prev = rooms[i-1].center();
        let new = room.center();

        if rng.range(0, 2) == 1 {
            apply_horizontal_tunnel(mb, prev.x, new.x, prev.y);
            apply_vertical_tunnel(mb, prev.y, new.y, new.x);
        } else {
            apply_vertical_tunnel(mb, prev.y, new.y, prev.x);
            apply_horizontal_tunnel(mb, prev.x, new.x, new.y);
        }
    }
}

impl MapArchitect for RoomsArchitect {
    fn new(&self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::blank();
        mb.fill(TileType::Wall);
        build_random_rooms(&mut mb, rng);
        build_corridors(&mut mb, rng);
 
        mb.player_start = mb.rooms[0].center();
        let furthest = mb.find_most_distant();
        mb.amulet_start = furthest;

        for r in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(r.center());
        }

        mb
    }
}