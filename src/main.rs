mod draw;
mod game;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use piston::Size;
use crate::game::Game;
use crate::draw::draw_game;

const SIZE: (i32, i32) = (20, 20);

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Snake", [512; 2])
        .graphics_api(opengl)
        .size(Size {width: 800.0, height: 800.0})
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");
  
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut game = Game::new(SIZE.0, SIZE.1);

    while let Some(e) = events.next(&mut window) {
        game.event(&e);
        
        if let Some(args) = e.render_args() {
            draw_game(&game, &args, &mut gl);
        }
    }
}

