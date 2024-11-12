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

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;
    }
    fn resize(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Ended;
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

fn main_menu(ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Dragon!");
    ctx.print_centered(8, "(P) Play Game!");
    ctx.print_centered(9, "(Q) Quit Game!");

}
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy dragon").build()?;
    main_loop(context, State::new())
}
