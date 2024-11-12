use bracket_lib::prelude::*;
enum GameMode {
    Menu,
    Playing,
    Ended,
}
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
       match self.mode {
           GameMode::Menu => self.main_menu(ctx),
           GameMode::Ended => self.dead(ctx),
           GameMode::Playing => self.play(ctx)
       }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy dragon").build()?;
    main_loop(context, State::new())
}
