extern crate termion;
use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::async_stdin;
use termion::raw::IntoRawMode;
use std::io::{stdout, Write, stdin, Read};
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
    width: u16,
    height: u16,
    stdout: T,
    stdin: F,
    snake: Snake,
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

    fn init_snake(&mut self) {
        write!(self.stdout,"{}{}", cursor::Goto(self.width/2, self.height/2), self.snake.body[0].part).unwrap();
        write!(self.stdout,"{}{}", cursor::Goto(self.width/2+1, self.height/2), self.snake.body[1].part).unwrap();
        self.stdout.flush().unwrap();
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
        self.print_field();
        self.print_snake();
    }

    fn take_direction(&mut self, dir: Direction) {
        let mut head = true;
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
                    Direction::Up => i.part = "||",
                    Direction::Down => i.part = "||",
                    Direction::Left => i.part = "=",
                    Direction::Right => i.part = "=",
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

    fn start_snake_game(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        self.print_field();
        self.init_snake();
        loop {
            self.move_snake();
            sleep(Duration::from_millis(300));
        }
    }
}



fn init() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = async_stdin();
    let mut game = Game{
        width: 60,
        height: 20,
        stdout: stdout,
        stdin: stdin,
        snake: Snake {
            body: vec![
                BodyPart{x: 60/2, y: 20/2, part: "<", direction: Direction::Left},
                BodyPart{x: 60/2, y: (20/2) + 1, part: "=", direction: Direction::Left}
            ]
        },
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

fn main() {
    init();
}
