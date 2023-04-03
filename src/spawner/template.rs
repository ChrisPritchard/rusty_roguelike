use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::{fs::File, collections::HashSet};
use legion::systems::CommandBuffer;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Item, Enemy
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("failed opening templates");
        from_reader(file).expect("failed to deserialize templates")
    }

    pub fn spawn_entities(&self, ecs: &mut World, rng: &mut RandomNumberGenerator, level: usize, spawn_points: &[Point]) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|e| {
                for _ in 0..e.frequency {
                    available_entities.push(e)
                }
            });
        let mut command_buffer = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|p| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(*p, entity, &mut command_buffer);
            }
        });
        let mut resources = Resources::default(); // change in api since book
        command_buffer.flush(ecs, &mut resources);
    }

    fn spawn_entity(&self, p: Point, template: &Template, command_buffer: &mut CommandBuffer) {
        let entity = command_buffer.push((
           p, Render{ color: ColorPair::new(WHITE, BLACK), glyph: to_cp437(template.glyph) }, Name(template.name.clone())
        ));
        match template.entity_type {
            EntityType::Item => command_buffer.add_component(entity, Item),
            EntityType::Enemy => {
                command_buffer.add_component(entity, Enemy);
                command_buffer.add_component(entity, FieldOfView::new(6));
                command_buffer.add_component(entity, ChasingPlayer);
                command_buffer.add_component(entity, Health::new(template.hp.unwrap()));
            }
        }

        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(effect, n)| {
                match effect.as_str() {
                    "Healing" => command_buffer.add_component(entity, ProvidesHealing{amount: *n}),
                    "MagicMap" => command_buffer.add_component(entity, ProvidesDungeonMap),
                    _ => println!("unknown item effect: {}", effect)
                }
            });
        }
    }
}
