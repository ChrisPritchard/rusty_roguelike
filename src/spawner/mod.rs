use crate::prelude::*;

use self::template::Templates;

mod template;

pub fn spawn_level(ecs: &mut World, rng: &mut RandomNumberGenerator, level: usize, spawn_points: &[Point]) {
    let templates = Templates::load();
    templates.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player {map_level: 0},
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

// pub fn spawn_drunk_goblin(ecs: &mut World, pos: Point) {
//     ecs.push(
//         (
//             Enemy,
//             MovesRandomly,
//             pos,
//             Render{
//                 color: ColorPair::new(WHITE, BLACK),
//                 glyph: to_cp437('g')
//             },
//             Name("Drunk Goblin".to_string()),
//             Health::new(2),
//             FieldOfView::new(4),
//         )
//     );
// }

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
