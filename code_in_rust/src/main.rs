use std::{fmt::{self, Display, Error, Formatter}, time::Duration};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 63,
                height: 31,
            },
            ball: Ball {
                x: 44,
                y: 21,
                vert_dir: VertDir::Down,
                horiz_dir: HorizDir::Right,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if frame.width <= self.x {
            self.horiz_dir = HorizDir::Left;
        }
        if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // Top border
        write!(f, "+")?;
        for _ in 0..self.frame.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;

        // Content
        for y in 0..self.frame.height {
            write!(f, "|")?;
            for x in 0..self.frame.width {
                if self.ball.x == x && self.ball.y == y {
                    write!(f, "o")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "|")?;
        }

        // Bottom border
        write!(f, "+")?;
        for _ in 0..self.frame.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;

        Ok(())
    }
}

fn main() {
    let mut new_game = Game::new();
    let sleep_time = Duration::from_millis(33);
    loop {
        println!("{}", new_game);
        new_game.step();
        std::thread::sleep(sleep_time);
        println!("{}  {}",new_game.ball.x, new_game.ball.y);
    }
}
