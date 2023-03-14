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

pub fn spawn_goblin(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Enemy,
            ChasingPlayer,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('g')
            },
            Name("Goblin".to_string()),
            Health::new(1)
        )
    );
}

pub fn spawn_drunk_goblin(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Enemy,
            MovesRandomly,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('g')
            },
            Name("Drunk Goblin".to_string()),
            Health::new(2)
        )
    );
}

pub fn spawn_orc(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Enemy,
            ChasingPlayer,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('o')
            },
            Name("Orc".to_string()),
            Health::new(2)
        )
    );
}