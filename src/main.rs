use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Rehelmeting {
    gl: GlGraphics,
}

impl Rehelmeting {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            use graphics::*;

            let rect = rectangle::rectangle_by_corners(350.0, 250.0, 450.0, 350.0);

            clear([0.0, 0.0, 0.0, 1.0], gl);
            rectangle([1.0, 0.0, 0.0, 1.0], rect, ctx.transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rehelmeting²", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Rehelmeting {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}
