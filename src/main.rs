extern crate termion;
extern crate rand;
use termion::{clear, cursor};
use termion::async_stdin;
use termion::raw::IntoRawMode;
use rand::Rng;
use std::io::{stdout, Write, stdin, Read};
use std::vec::Vec;
use std::thread::sleep;
use std::time::Duration;
use std::process::exit;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct BodyPart {
    x: u16,
    y: u16,
    part: &'static str,
    direction: Direction
}

pub struct Snake {
    body: Vec<BodyPart>
}

pub struct Game<T,F> {
    stdout: T,
    stdin: F,
    snake: Snake,
    food: (u16,u16),
    field: [[char; 60]; 20]
}

impl<T: Write,F: Read> Game<T,F>{
   /**********************************************************************
    *                      Creates and resets field                      *
    **********************************************************************/
    fn print_field(&mut self) {
        write!(self.stdout,"{}{}", clear::All, cursor::Goto(1,1)).unwrap();
        self.stdout.flush().unwrap();
        for i in 0..20 {
            for j in 0..60 {
                write!(self.stdout,"{}", self.field[i][j]).unwrap();
            }
            write!(self.stdout, "{}\n", cursor::Goto(1,(i+1) as u16)).unwrap();
        }
    }

    fn move_snake(&mut self) {
        let mut key = [0];
        self.stdin.read(&mut key).unwrap();
        match key[0]{
            b'q' => exit(0),
            b'w' | b'k' if self.snake.body[0].direction != Direction::Down => self.take_direction(Direction::Up),
            b'a' | b'h' if self.snake.body[0].direction != Direction::Right => self.take_direction(Direction::Left),
            b'd' | b'l' if self.snake.body[0].direction != Direction::Left => self.take_direction(Direction::Right),
            b's' | b'j' if self.snake.body[0].direction != Direction::Up => self.take_direction(Direction::Down),
            _ => {},
        }
        self.check_food();
        self.print_field();
        self.print_snake();
        self.print_food();
    }

    fn take_direction(&mut self, dir: Direction) {
        let mut head = true;
        for i in (0..self.snake.body.len()).rev() {
            if i != 0 {
                self.snake.body[i].direction = self.snake.body[i-1].direction;
                self.snake.body[i].x = self.snake.body[i-1].x;
                self.snake.body[i].y = self.snake.body[i-1].y;
            }
        }
        for i in &mut self.snake.body {
            if head==true {
                match dir {
                    Direction::Up => {
                        i.part = "^";
                        i.y -= 1;
                    },
                    Direction::Down => {
                        i.part = "v";
                        i.y += 1;
                    },
                    Direction::Left => {
                        i.part = "<";
                        i.x -= 1;
                    },
                    Direction::Right => {
                        i.part = ">";
                        i.x += 1;
                    },
                }
                i.direction = dir;
                head = false;
            }
            else {
                match i.direction {
                    Direction::Up => i.part = "║",
                    Direction::Down => i.part = "║",
                    Direction::Left => i.part = "═",
                    Direction::Right => i.part = "═",
                }
            }
        }
    }

    fn print_snake(&mut self) {
        for i in self.snake.body.iter() {
            write!(self.stdout,"{}{}", cursor::Goto(i.x, i.y), i.part).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn check_game_over(&mut self) -> bool {
        for i in 0..60 {
            if self.snake.body[0].x == i &&
                (self.snake.body[0].y == 1 || self.snake.body[0].y == 20) {
                return true;
            }
        }
        for i in 0..20 {
            if self.snake.body[0].y == i &&
                (self.snake.body[0].x == 1 || self.snake.body[0].x == 60) {
                return true;
            }
        }
        false
    }

    fn check_food(&mut self) {
        if self.snake.body[0].x == self.food.0 &&
            self.snake.body[0].y == self.food.1 {
                self.food = food_gen();
            }
    }

    fn print_food(&mut self) {
        let food = "×";
        write!(self.stdout, "{}{}", cursor::Goto(self.food.0, self.food.1), food).unwrap();
        self.stdout.flush().unwrap();
    }



    fn start_snake_game(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        self.print_field();
        self.print_snake();
        loop {
            self.move_snake();
            if self.check_game_over() {break};
            sleep(Duration::from_millis(100));
        }
        let mut stdin = stdin();
        let mut key = [0];
        stdin.read(&mut key).unwrap();
        match key[0] {
            b'q' | b'Q' => exit(0),
            b'r' | b'R' => init(),
            _ => {}
        }
    }
}



fn init() {
    let stdout = stdout().into_raw_mode().unwrap();
    let stdin = async_stdin();
    let mut game = Game{
        stdout: stdout,
        stdin: stdin,
        snake: Snake {
            body: vec![
                BodyPart{x: 60/2, y: 20/2, part: "<", direction: Direction::Left},
                BodyPart{x: 60/2 + 1, y: (20/2), part: "═", direction: Direction::Left}
            ]
        },
        food: food_gen(),
        field: init_array()
    };
    game.start_snake_game();
}

fn init_array() -> [[char; 60]; 20] {
    let mut field: [[char; 60];20] = [[' '; 60];20];
    for i in 0..60 {
        field[0][i] = '#';
        field[19][i] = '#';
    }

    for i in 0..20 {
        field[i][0] = '#';
        field[i][59] = '#';
    }
    field
}

fn food_gen() -> (u16, u16) {
    //let food = "×";
    let rx = rand::thread_rng().gen_range(2, 60);
    let ry = rand::thread_rng().gen_range(2, 20);
    (rx, ry)
}

fn main() {
    init();
}
