extern crate termion;
extern crate rand;
use termion::{clear, cursor, style, async_stdin, color};
use termion::raw::IntoRawMode;
use rand::Rng;
use std::io::{stdout, Write, Read};
use std::vec::Vec;
use std::thread::sleep;
use std::time::Duration;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Copy, Clone)]
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
    score: i32,
    highscore: i32,
    speed: u64,
    field: [[char; 60]; 20]
}

impl<T: Write,F: Read> Game<T,F>{
   /**********************************************************************
    *                      Creates and resets field                      *
    **********************************************************************/
    fn print_field(&mut self) {
        write!(self.stdout,"{}{}{}", clear::All, cursor::Goto(1,1), color::Fg(color::Blue)).unwrap();
        self.stdout.flush().unwrap();
        for i in 0..20 {
            for j in 0..60 {
                write!(self.stdout,"{}", self.field[i][j]).unwrap();
            }
            write!(self.stdout, "{}\n", cursor::Goto(1,(i+1) as u16)).unwrap();
        }
        write!(self.stdout,"{}", color::Fg(color::Reset)).unwrap();
        self.stdout.flush().unwrap();
    }

   /**********************************************************************
    *       w,a,s,d or h,j,k,l to move snake and redraw everything       *
    **********************************************************************/
    fn move_snake(&mut self) -> bool {
        let mut key = [0];
        self.stdin.read(&mut key).unwrap();
        match key[0]{
            b'q' | b'Q' => return false,
            b'w' | b'k' if self.snake.body[0].direction != Direction::Down
                && self.snake.body[0].direction != Direction::Up=> self.take_direction(Direction::Up),
            b'a' | b'h' if self.snake.body[0].direction != Direction::Right
                && self.snake.body[0].direction != Direction::Left=> self.take_direction(Direction::Left),
            b'd' | b'l' if self.snake.body[0].direction != Direction::Left
                && self.snake.body[0].direction != Direction::Right => self.take_direction(Direction::Right),
            b's' | b'j' if self.snake.body[0].direction != Direction::Up
                && self.snake.body[0].direction != Direction::Down=> self.take_direction(Direction::Down),
            _ => self.automove(),
        }
        self.check_food();
        self.print_snake();
        return true;
    }

   /**********************************************************************
    *                   keeps the snake moving                           *
    **********************************************************************/
    fn automove(&mut self) {
        let dir = self.snake.body[0].direction.clone();
        self.take_direction(dir);
    }

   /**********************************************************************
    * change direction of the snake to all parts and make it move        *
    **********************************************************************/
    fn take_direction(&mut self, dir: Direction) {
        let mut head = true;
        write!(self.stdout, "{} ",cursor::Goto(self.snake.body[self.snake.body.len()-1].x,self.snake.body[self.snake.body.len()-1].y))
            .unwrap();
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
                        i.part = "▲";
                        i.y -= 1;
                    },
                    Direction::Down => {
                        i.part = "▼";
                        i.y += 1;
                    },
                    Direction::Left => {
                        i.part = "◀";
                        i.x -= 1;
                    },
                    Direction::Right => {
                        i.part = "▶";
                        i.x += 1;
                    },
                }
                i.direction = dir;
                head = false;
            }
            else {
                match i.direction {
                    Direction::Up => i.part = "▪",
                    Direction::Down => i.part = "▪",
                    Direction::Left => i.part = "▪",
                    Direction::Right => i.part = "▪",
                }
            }
        }
    }

   /**********************************************************************
    *      when snake eats food it grows by one part                     *
    **********************************************************************/
    fn grow_snake(&mut self) {
        let snake_size = self.snake.body.len() - 1;
        let tail = self.snake.body[snake_size].clone();
        match tail.direction {
            Direction::Up => {
                self.snake.body.push(BodyPart {
                    x: tail.x, y: tail.y - 1, part: "▪", direction: tail.direction
                });
            },
            Direction::Down => {
                self.snake.body.push(BodyPart {
                    x: tail.x, y: tail.y + 1, part: "▪", direction: tail.direction
                });
            },
            Direction::Right => {
                self.snake.body.push(BodyPart {
                    x: tail.x - 1, y: tail.y, part: "▪", direction: tail.direction
                });
            },
            Direction::Left => {
                self.snake.body.push(BodyPart {
                    x: tail.x + 1, y: tail.y, part: "▪", direction: tail.direction
                });
            },
        }
        self.score += 10;
        if self.score > self.highscore {
            self.highscore = self.score;
        }
        if self.speed > 140 {
            self.speed -= 20;
        }
        self.print_score();
        self.print_food();
    }

    fn print_score(&mut self) {
        write!(self.stdout,"{}{}Hi-Score: {}{}", cursor::Goto(70, 5), color::Fg(color::Green), self.highscore, color::Fg(color::Reset)).unwrap();
        write!(self.stdout,"{}{}Score: {}{}", cursor::Goto(70, 6), color::Fg(color::Green), self.score, color::Fg(color::Reset)).unwrap();
        self.stdout.flush().unwrap();
    }

   /**********************************************************************
    *                         reprint snake                              *
    **********************************************************************/
    fn print_snake(&mut self) {
        for i in self.snake.body.iter() {
            write!(self.stdout,"{}{}{}{}", cursor::Goto(i.x, i.y), color::Fg(color::Green), i.part, color::Fg(color::Reset)).unwrap();
        }
        self.stdout.flush().unwrap();
    }

   /**********************************************************************
    *               check if snake hit a wall or itself                  *
    **********************************************************************/
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
        let mut head = true;
        for i in self.snake.body.iter() {
            if head == false {
                if self.snake.body[0].x == i.x &&
                    self.snake.body[0].y == i.y {
                        return true;
                    }
            }
            head = false;
        }
        false
    }

   /**********************************************************************
    *          check if snake found food to eat                          *
    **********************************************************************/
    fn check_food(&mut self) {
        if self.snake.body[0].x == self.food.0 &&
            self.snake.body[0].y == self.food.1 {
                self.food = food_gen();
                loop {
                    if self.validate_food() {break;}
                }
                self.grow_snake();
            }
    }

    fn validate_food(&mut self) -> bool {
        for i in self.snake.body.iter() {
            if i.x == self.food.0
                && i.y == self.food.1 {
                    self.food = food_gen();
                    return false;
                }
        }
        true
    }

   /**********************************************************************
    *              reprint food every time                               *
    **********************************************************************/
    fn print_food(&mut self) {
        let food = "×";
        write!(self.stdout, "{}{}{}{}", cursor::Goto(self.food.0, self.food.1), color::Fg(color::Red), food, color::Fg(color::Reset)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn print_game_over(&mut self) {
        write!(self.stdout,"{}-------------------------", cursor::Goto((60/2) -10, 20/2 - 3)).unwrap();
        write!(self.stdout,"{}|       Game Over!      |", cursor::Goto((60/2) -10, 20/2 - 2)).unwrap();
        write!(self.stdout,"{}|     HighScore: {}    |", cursor::Goto((60/2) -10, 20/2 - 1), self.score).unwrap();
        write!(self.stdout,"{}|       Score: {}      |", cursor::Goto((60/2) -10, 20/2), self.score).unwrap();
        write!(self.stdout,"{}|(r)etry          (q)uit|", cursor::Goto((60/2) -10, 20/2 + 1)).unwrap();
        write!(self.stdout,"{}-------------------------", cursor::Goto((60/2) -10, 20/2 + 2)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn start_snake_game(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        self.print_field();
        self.print_score();
        self.print_food();
        self.print_snake();
        loop {
            if !self.move_snake() {
                break;
            }
            if self.check_game_over() {break};
            sleep(Duration::from_millis(self.speed));
        }
        if !self.end_game() {
            self.start_snake_game();
        }
    }

    fn end_game(&mut self) -> bool {
        self.print_game_over();
        loop {
            let mut key = [0];
            self.stdin.read(&mut key).unwrap();
            match key[0] {
                b'r' | b'R' => {
                    self.snake.body = vec![
                        BodyPart{x: 60/2, y: 20/2, part: "◀", direction: Direction::Left},
                        BodyPart{x: 60/2 + 1, y: (20/2), part: "▪", direction: Direction::Left}
                    ];
                    self.score = 0;
                    self.food = food_gen();
                    self.speed = 300;
                    return false;
                },
                b'q' | b'Q'=> {
                    write!(self.stdout, "{}{}{}", clear::All, style::Reset, cursor::Show).unwrap();
                    self.stdout.flush().unwrap();
                    return true;
                }
                _ => {},
            }
        }
    }
}

/**********************************************************************
 *               initialize everything(snake, game, score)            *
 **********************************************************************/
fn init() {
    let stdout = stdout().into_raw_mode().unwrap();
    let stdin = async_stdin();
    let mut game = Game{
        stdout: stdout,
        stdin: stdin,
        snake: Snake {
            body: vec![
                BodyPart{x: 60/2, y: 20/2, part: "◀", direction: Direction::Left},
                BodyPart{x: 60/2 + 1, y: (20/2), part: "▪", direction: Direction::Left}
            ]
        },
        food: food_gen(),
        score: 0,
        highscore: 0,
        speed: 300,
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

/**********************************************************************
 *               generate food at a random location                   *
 **********************************************************************/
fn food_gen() -> (u16, u16) {
    let rx = rand::thread_rng().gen_range(2, 60);
    let ry = rand::thread_rng().gen_range(2, 20);
    (rx, ry)
}

fn main() {
    init();
}
