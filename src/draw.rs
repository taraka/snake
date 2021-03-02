use crate::game::{Game, State};

use graphics::*; 
use opengl_graphics::GlGraphics;
use piston::RenderArgs; 
use graphics::types::Color;

const COLOR_BACKGROUND: Color = [0.6, 0.6, 0.6, 1.0];
const COLOR_GRID_DEAD: Color = [0.0, 0.0, 0.0, 1.0];
const COLOR_APPLE: Color = [0.8, 0.2, 0.2, 1.0];
const COLOR_SNAKE: Color = [0.2, 0.8, 0.2, 1.0];
const COLOR_SNAKE_HEAD: Color = [0.1, 0.6, 0.2, 1.0];
const COLOR_GRID: Color = [0.3, 0.3, 0.3, 1.0];
const PADDING: f64 = 10.0;

pub fn draw_game(game: &Game, args: &RenderArgs, gl: &mut GlGraphics) {
    gl.draw(args.viewport(), |c, g| {
        clear(COLOR_BACKGROUND, g);

        let grid_width = args.window_size[0] - 2.0 * PADDING;
        let grid_height = args.window_size[1] - 2.0 * PADDING;

        let cell_width = grid_width / game.width as f64;
        let cell_height = grid_height / game.height as f64;
        
        let grid_color = match game.state {
            State::DEAD => COLOR_GRID_DEAD,
            _ => COLOR_GRID
        };
        Rectangle::new(grid_color)
            .draw([PADDING, PADDING, grid_width, grid_height], &c.draw_state, c.transform, g);

        // Draw Snake
        Rectangle::new(COLOR_SNAKE_HEAD)
                .draw([PADDING + (game.snake.head.0 as f64 * cell_width), PADDING + (game.snake.head.1 as f64 * cell_height), cell_width, cell_height], &c.draw_state, c.transform, g);
        for i in game.snake.body.iter() {
            Rectangle::new(COLOR_SNAKE)
                .draw([PADDING + (i.0 as f64 * cell_width), PADDING + (i.1 as f64 * cell_height), cell_width, cell_height], &c.draw_state, c.transform, g);
        }

        // Draw Apple
        Rectangle::new(COLOR_APPLE)
                .draw([PADDING + (game.apple.0 as f64 * cell_width), PADDING + (game.apple.1 as f64 * cell_height), cell_width, cell_height], &c.draw_state, c.transform, g);

    });
    
}