use std::collections::LinkedList;
use piston::input::{GenericEvent,Button, Key};
use rand::Rng;

pub type SnakeCell = (i32, i32);

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq, Debug)]
pub enum State {
    RUNNING,
    DEAD,
    WIN
}

pub struct Game {
    pub snake: Snake,
    pub apple: SnakeCell,
    pub height: i32,
    pub width: i32,
    last_update: f64,
    tick_time: f64,
    state: State,
    input_queue: LinkedList<Key>,
}

impl Game {
    pub fn new(height: i32
, width: i32) -> Self {
        let mut game = Self {
            snake: Snake::new(),
            apple: (0, 0),
            height: height,
            width: width,
            last_update: 0.0,
            tick_time: 0.4,
            state: State::RUNNING,
            input_queue: LinkedList::new(),

        };

        game.new_apple();

        game
    }  

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

        //Game Timing
        if let Some(args) = e.update_args() {
            self.last_update += args.dt;

            if self.last_update >= self.tick_time {
                self.last_update = 0.0;
                if self.state == State::RUNNING {
                    self.tick();
                }
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W | Key::A | Key::S | Key::D => self.input_queue.push_back(key),
                _ => ()
            }
        }
    }
    
    fn new_apple(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            self.apple.0 = rng.gen_range(0..self.width);
            self.apple.1 = rng.gen_range(0..self.height);

            if !self.snake.body.contains(&(self.apple.0,self.apple.1)) {
                break;
            }
        }
    }

    fn tick(&mut self) {

        if let Some(key) = self.input_queue.pop_front() {
            match key {
                Key::W => self.snake.direction = Direction::UP,
                Key::A => self.snake.direction = Direction::LEFT,
                Key::S => self.snake.direction = Direction::DOWN,
                Key::D => self.snake.direction = Direction::RIGHT,
                _ => ()
            }
        }

        let new_head = match self.snake.direction {
            Direction::UP => (self.snake.head.0, self.snake.head.1 - 1),
            Direction::DOWN => (self.snake.head.0, self.snake.head.1 + 1),
            Direction::LEFT => (self.snake.head.0 - 1, self.snake.head.1),
            Direction::RIGHT => (self.snake.head.0 + 1, self.snake.head.1),
        };

        // Hit a wall
        if new_head.0 >= self.width || 
            new_head.0 < 0 ||
            new_head.1 >= self.height ||
            new_head.1 < 0 {
                self.die();
                return;
            }

        // Hit self

        // Eat apple
        let eat_apple = new_head == self.apple;
        
        //Move Snake
        self.snake.advance(new_head, eat_apple);

        if eat_apple {
            self.new_apple();
            self.tick_time = self.tick_time * 0.9;
        }
    }

    fn die(&mut self) {
        self.state = State::DEAD;
    }
}



pub struct Snake {
    pub body: LinkedList<SnakeCell>,
    pub head: SnakeCell,
    pub direction: Direction
}

impl Snake {
    pub fn new() -> Self {
        let mut body = LinkedList::new();
        body.push_back((1,0));
        body.push_back((0,0));
        Self {
            body: body,
            head: (2,0),
            direction: Direction::RIGHT
        }
    }

    pub fn advance(&mut self, new_head:SnakeCell, should_grow: bool) {
        if !should_grow {
            self.body.pop_back();
        }
        self.body.push_front(self.head);
        self.head = new_head;
    }
    
}