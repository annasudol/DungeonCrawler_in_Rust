#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;
struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    //The function, to_cp437(), converts a Unicode symbol from our source code to the matching Codepage-437 character number.
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x + 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
}
enum GameMode {
    Menu,
    Playing,
    End,
}

// START: state
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    // START: restart
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        // END: mainprint
        // START: mainiflet

        if let Some(key) = ctx.key {// <callout id="co.flappy_state.if_let_key" />
            match key {
                VirtualKeyCode::P => self.restart(),// <callout id="co.flappy_state.restart" />
                VirtualKeyCode::Q => ctx.quitting = true,// <callout id="co.flappy_state.quitting" />
                _ => {}
            }
        }
    } // Close the function
    // END: mainiflet
    // END: mainmenu

    // START: dead
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    // END: dead

    // START: play
    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }
    // END: play
}

// START: tick
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}
// END: tick

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    // START: mainloop
    main_loop(context, State::new())
    // END: mainloop
}
