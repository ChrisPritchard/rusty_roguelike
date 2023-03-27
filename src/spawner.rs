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
            Name("You".to_string()),
            Health::new(10),
            FieldOfView::new(8),
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
            Health::new(1),
            FieldOfView::new(6),
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
            Health::new(2),
            FieldOfView::new(4),
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
            Health::new(2),
            FieldOfView::new(6),
        )
    );
}

pub fn spawn_amulet_of_yara(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Item,
            AmuletOfYara,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|')
            },
            Name("Amulet of Yara".to_string()),
        )
    );
}

pub fn spawn_health_potion(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Item,
            ProvidesHealing{amount: 6},
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('!')
            },
            Name("Healing Potion".to_string()),
        )
    );
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Item,
            ProvidesDungeonMap,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('{')
            },
            Name("Dungeon Map".to_string()),
        )
    );
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    match rng.roll_dice(1, 6) {
        1 => spawn_health_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        3 => spawn_orc(ecs, pos),
        4 => spawn_drunk_goblin(ecs, pos),
        _ => spawn_goblin(ecs, pos)
    }
}