#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;
use rand::random;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new() -> Self {
        let mut random = RandomNumberGenerator::new()
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('|'),
            );
        }
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('|'),
            );
        }
    }
}
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

    fn flap(&mut self) {
        self.velocity = -2.0;
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
    player: Player,
    frame_time: f32
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(1, 0),
        }
    }
    // START: restart
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(1, 0);
        self.frame_time = 0.0;
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
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0, "Press SPACE to flap!");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
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
