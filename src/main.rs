mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true;
        }
    }
}

impl State {
    fn new() -> Self {
        State { 
            map: Map::new(), 
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)) 
        }
    }
}

fn main() -> BError {
    let context = 
        BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Dungeoncrawl")
        .with_fullscreen(true)
        .build()?;
    main_loop(context, State::new())
}
