use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(ecs: &mut SubWorld, 
        #[resource] map: &Map, 
        #[resource] key: &Option<VirtualKeyCode>, 
        #[resource] camera: &mut Camera) {

    if let Some(key) = key {
        let delta = 
            match key {
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Left => Point::new(-1, 0),
                _ => return,
            };
        let mut players = <&mut Point>::query().filter(component::<Player>());
        players.iter_mut(ecs).for_each(|pos| {
            let next_position = *pos + delta;
            if map.can_enter_tile(next_position) {
                *pos = next_position;
                camera.on_player_move(next_position);
            }
        });
    }

}