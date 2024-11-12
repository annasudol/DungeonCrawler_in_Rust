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
        ctx.cls();
        ctx.print(1,1,"Hello, world!");
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy dragon").build()?;
    main_loop(context, State::new())
}
