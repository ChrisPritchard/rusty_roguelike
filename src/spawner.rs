use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health { current: 20, max: 20 }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, pos: Point, monster_type: i32) {
    ecs.push(
        (
            Enemy,
            MovesRandomly,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: match monster_type {
                    0 => to_cp437('E'),
                    1 => to_cp437('O'),
                    2 => to_cp437('o'),
                    _ => to_cp437('g'),
                }
            }
        )
    );
}