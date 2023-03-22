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
}