use crate::game::{Game, State, Direction};

use graphics::{text::Text, Rectangle, Ellipse, clear}; 
use graphics::character::CharacterCache;
use graphics::{Context, Graphics, Transformed};
use piston::RenderArgs; 
use graphics::types::Color;

const COLOR_BACKGROUND: Color = [0.6, 0.85, 9.0, 1.0];
const COLOR_APPLE: Color = [0.9, 0.3, 0.3, 1.0];
const COLOR_SNAKE: Color = [0.2, 0.7, 0.2, 1.0];
const COLOR_SNAKE_DEAD: Color = [0.35, 0.2, 0.03, 1.0];
const COLOR_SNAKE_HEAD: Color = [0.1, 0.6, 0.2, 1.0];
const COLOR_SNAKE_BOOST: Color = [0.5, 0.1, 0.1, 1.0];
const COLOR_SNAKE_EYES: Color = [0.0, 0.0, 0.0, 1.0];
const COLOR_GRID: Color = [0.9, 1.0, 0.9, 1.0];
const PADDING: f64 = 10.0;

pub fn draw_game<G: Graphics, C>(game: &Game, args: &RenderArgs, glyphs: &mut C, c: &Context, g: &mut G)
    where C: CharacterCache<Texture = G::Texture> {

    clear(COLOR_BACKGROUND, g);

    let header_height: f64 = 40.0 + (2.0 * PADDING);

    let grid_width = args.window_size[0] - 2.0 * PADDING;
    let grid_height = args.window_size[1] - 2.0 * PADDING - header_height;

    let cell_width = grid_width / game.width as f64;
    let cell_height = grid_height / game.height as f64;
    

    //Draw Header
    let size = 32;
    let transform = c.transform.trans(PADDING, PADDING+32.0);
    let score = (game.snake.body.len() - 2) * 10;
    Text::new_color(COLOR_GRID, size)
        .draw(&format!("Score: {}", score), glyphs, &c.draw_state, transform, g).map_err(|_| "Error").unwrap();

     let transform = c.transform.trans(500.0, PADDING+32.0);
    let state_string = match game.state {
        State::DEAD => "You died!",
        State::PAUSED => "Paused (p)",
        State::START => "Press P to start",
        _ => ""
    };
    Text::new_color(COLOR_GRID, size)
        .draw(&format!("{}", state_string), glyphs, &c.draw_state, transform, g).map_err(|_| "Error").unwrap();

    // Draw Grid
    
    Rectangle::new(COLOR_GRID)
        .draw([PADDING, header_height, grid_width, grid_height], &c.draw_state, c.transform, g);

    // Draw Snake Head
    let head_color = match game.boost {
        true => COLOR_SNAKE_BOOST,
        false => COLOR_SNAKE_HEAD
    };
    Rectangle::new(head_color)
            .draw([PADDING + (game.snake.head.0 as f64 * cell_width)+1.0, header_height + (game.snake.head.1 as f64 * cell_height)+1.0, cell_width-1.0, cell_height-1.0], &c.draw_state, c.transform, g);

    if game.snake.direction == Direction::RIGHT || game.snake.direction == Direction::DOWN {
        Ellipse::new(COLOR_SNAKE_EYES)
            .draw([PADDING + (game.snake.head.0 as f64 * cell_width)+1.0 + 4.0*cell_width/6.0, header_height + (game.snake.head.1 as f64 * cell_height)+1.0 + 4.0*cell_height/6.0, cell_width/6.0, cell_height/6.0], &c.draw_state, c.transform, g);
    }
    if game.snake.direction == Direction::LEFT || game.snake.direction == Direction::DOWN {
        Ellipse::new(COLOR_SNAKE_EYES)
            .draw([PADDING + (game.snake.head.0 as f64 * cell_width)+1.0 + 1.0*cell_width/6.0, header_height + (game.snake.head.1 as f64 * cell_height)+1.0 + 4.0*cell_height/6.0, cell_width/6.0, cell_height/6.0], &c.draw_state, c.transform, g);
    }
    if game.snake.direction == Direction::RIGHT || game.snake.direction == Direction::UP {
        Ellipse::new(COLOR_SNAKE_EYES)
            .draw([PADDING + (game.snake.head.0 as f64 * cell_width)+1.0 + 4.0*cell_width/6.0, header_height + (game.snake.head.1 as f64 * cell_height)+1.0 + 1.0*cell_height/6.0, cell_width/6.0, cell_height/6.0], &c.draw_state, c.transform, g);
    }
    if game.snake.direction == Direction::LEFT || game.snake.direction == Direction::UP {
        Ellipse::new(COLOR_SNAKE_EYES)
            .draw([PADDING + (game.snake.head.0 as f64 * cell_width)+1.0 + 1.0*cell_width/6.0, header_height + (game.snake.head.1 as f64 * cell_height)+1.0 + 1.0*cell_height/6.0, cell_width/6.0, cell_height/6.0], &c.draw_state, c.transform, g);
    }

    

    // Draw Body
    let snake_color = match game.state {
        State::DEAD => COLOR_SNAKE_DEAD,
        _ => COLOR_SNAKE
    };
    for i in game.snake.body.iter() {
        Rectangle::new(snake_color)
            .draw([PADDING + (i.0 as f64 * cell_width) + 1.0, header_height + (i.1 as f64 * cell_height)+1.0, cell_width-1.0, cell_height-1.0], &c.draw_state, c.transform, g);
    }

    // Draw Apple
    Ellipse::new(COLOR_APPLE)
            .draw([PADDING + (game.apple.0 as f64 * cell_width), header_height + (game.apple.1 as f64 * cell_height), cell_width, cell_height], &c.draw_state, c.transform, g);

    
}