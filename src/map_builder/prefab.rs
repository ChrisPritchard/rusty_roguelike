use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
", 12, 11); 

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra = mb.map.dijstra_map(mb.player_start);

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1), 
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2), 
            FORTRESS.1, FORTRESS.2);

        let mut can_place = true;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let dist = dijkstra.map[idx];
            if dist == f32::MAX || dist <= 20. || mb.amulet_start == pt {
                can_place = false;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain (|pt| !points.contains(pt));
        }
        attempts += 1;
    }

    if placement.is_none() {
        return;
    }

    let placement = placement.unwrap();
    let fort_chars = FORTRESS.0.chars().filter(|c| *c != '\r' && *c != '\n').collect::<Vec<char>>();
    let mut i = 0;
    for ty in placement.y..placement.y + FORTRESS.2 {
        for tx in placement.x..placement.x + FORTRESS.1 {
            let idx = map_idx(tx, ty);
            let c = fort_chars[i];
            match c {
                'M' => {
                    mb.map.tiles[idx] = TileType::Floor;
                    mb.monster_spawns.push(Point::new(tx, ty));
                },
                '-' => mb.map.tiles[idx] = TileType::Floor,
                '#' => mb.map.tiles[idx] = TileType::Wall,
                _ => (),
            }
            i += 1;
        }
    }
}