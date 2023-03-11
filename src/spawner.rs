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
            Health::new(20)
        )
    );
}

pub fn spawn_monster(ecs: &mut World, pos: Point, monster_type: i32) {
    let (hp, name, glyph) = match monster_type {
        0 => spawn_goblin(),
        _ => spawn_orc(),
    };

    ecs.push(
        (
            Enemy,
            MovesRandomly,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph
            },
            name,
            hp
        )
    );
}

fn spawn_goblin() -> (Health, Name, FontCharType) {
    (Health::new(1), Name("Goblin".to_string()), to_cp437('g'))
}

fn spawn_orc() -> (Health, Name, FontCharType) {
    (Health::new(2), Name("Orc".to_string()), to_cp437('o'))
}