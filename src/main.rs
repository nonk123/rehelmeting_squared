use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod dungeon;

use dungeon::{Dungeon, Tile};

struct Rehelmeting {
    dungeon: Dungeon,
    /// `true` if a new dungeon is requested.
    new_dungeon: bool,
}

impl Rehelmeting {
    fn new() -> Self {
        Self {
            dungeon: Dungeon::new(),
            new_dungeon: true,
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |ctx, gl| {
            use graphics::*;

            clear([0.0, 0.0, 0.0, 1.0], gl);

            let tile_size = 20f64;

            for x in 0..dungeon::WIDTH {
                for y in 0..dungeon::HEIGHT {
                    match self.dungeon.tiles[y][x] {
                        Tile::Empty => continue,
                        _ => {}
                    }

                    let color = [0.3, 0.3, 0.3, 1.0];

                    let x0 = x as f64 * tile_size;
                    let y0 = y as f64 * tile_size;

                    let x1 = x0 + tile_size;
                    let y1 = y0 + tile_size;

                    let rect = rectangle::rectangle_by_corners(x0, y0, x1, y1);

                    rectangle(color, rect, ctx.transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.new_dungeon {
            self.dungeon.generate();
            self.new_dungeon = false;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rehelmeting²", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Rehelmeting::new();

    let mut events = Events::new(EventSettings::new());

    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::G => game.new_dungeon = true,
                Key::Q => break,
                _ => {}
            }
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.render_args() {
            game.render(&mut gl, &args);
        }
    }
}
