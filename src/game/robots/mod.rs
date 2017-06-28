use std::path::Path;

mod score_table;

use self::score_table::ScoreTable;

use game::{Game, InputState, TickResult};

use imprint::{Imprint, Cell};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub const MAX_LEVEL: u32 = ::FRAMERATE; // should always be <= FRAMERATE
pub const MAX_ROBOTS: u32 = 20; // should always be <= FRAMERATE

pub struct Config {
    pub robots: u32,
    pub level: u32,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Paused,
    Raising(usize),
    Lowering(usize),
    Menu(u32),
    Teleporting((usize, usize)),
}


pub struct Robots<'a> {
    pub config: Config,
    pub status: Status,
    pub anim_tick: u32,
    pub position: (usize, usize),
    movement_tick: u32,
    input: InputState,
    board: Imprint<CellData>,
    pub robots: Vec<(usize, usize)>,
    pub teleports: u32,
    points: u32,
    score_table: ScoreTable<'a>,
    speed: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellData {
    ScrapHeap,
    Robot,
}

impl<'a> Robots<'a> {
    pub fn new(filename: &'a Path) -> ::std::io::Result<Self> {
        let mut g = Robots {
            config: Config {
                robots: 5,
                level: 0,
            },
            status: Status::Menu(0),
            board: Imprint::empty(WIDTH, HEIGHT),
            movement_tick: 0,
            speed: MAX_LEVEL - 9,
            position: (WIDTH / 2, HEIGHT / 2),
            robots: Vec::new(),
            points: 0,
            teleports: 8,
            score_table: ScoreTable::new(filename)?,
            input: InputState::new(),
            anim_tick: 0,
        };
        g.new_game();
        Ok(g)
    }

    fn new_game(&mut self) {
        self.board = Imprint::empty(WIDTH, HEIGHT);
        self.score_table
            .update_scores(&self.config, self.points)
            .unwrap();
        self.position = (WIDTH / 2, HEIGHT / 2);
        self.points = 0;
        self.teleports = 8;
        self.robots = Vec::new();
        self.speed = MAX_LEVEL - self.config.level;
        for _ in 0..self.config.robots {
            let p = self.random_border_spot();
            self.robots.push(p);
            self.place_robot(p);
        }
    }
    fn place_robot(&mut self, p: (usize, usize)) {
        if self.board[p].is_empty() {
            self.board[p] = Cell::Filled(CellData::Robot);
        } else {
            self.board[p] = Cell::Filled(CellData::ScrapHeap);
        }
    }
    fn clear_robots(&mut self) {
        for p in &self.robots {
            self.board[*p] = Cell::Empty
        }
    }
    fn validate_robots(&mut self) -> usize {
        let x: usize = self.robots.len();
        let board = &self.board;
        self.robots.retain(|&p| {
            board[p] != Cell::Filled(CellData::ScrapHeap)
        });
        return x - self.robots.len();
    }
    fn random_border_spot(&self) -> (usize, usize) {
        let xc = ::rand::random::<u32>() as usize % WIDTH;
        let yc = ::rand::random::<u32>() as usize % HEIGHT;
        let b = ::rand::random::<u32>() % 4;
        let (x, y) = match b {
            0 => (xc, 0),
            1 => (xc, HEIGHT - 1),
            2 => (0, yc),
            _ => (WIDTH - 1, yc),
        };
        return (x, y);
    }

    fn random_free_spot(&self) -> (usize, usize) {
        let x = ::rand::random::<u32>() as usize % WIDTH;
        let y = ::rand::random::<u32>() as usize % HEIGHT;
        if !self.board[(x, y)].is_empty() || self.position == (x, y) {
            for xo in 0..WIDTH {
                for yo in 0..HEIGHT {
                    let new = ((x + xo) % WIDTH, (y + yo) % HEIGHT);
                    if self.board[new].is_empty() && new != self.position {
                        return new;
                    }
                }
            }
        }
        return (x, y);
    }

    fn towards((fx, fy): (usize, usize), (tx, ty): (usize, usize)) -> (usize, usize) {
        let x = if fx < tx {
            fx + 1
        } else if fx > tx {
            fx - 1
        } else {
            fx
        };
        let y = if fy < ty {
            fy + 1
        } else if fy > ty {
            fy - 1
        } else {
            fy
        };
        (x, y)
    }

    fn check_safety(&mut self) {
        if !self.board[self.position].is_empty() {
            self.status = Status::Raising(0);
        }
    }
    fn obliterate(&mut self, x0 : usize, y0 : usize, x1 : usize, y1 : usize) {
        for x in x0..x1+1 {
            for y in y0..y1+1 {
                if !self.board[(x,y)].is_empty() {
                    self.board[(x,y)] = Cell::Filled(CellData::ScrapHeap);
                }
            }
        }
        self.points += self.validate_robots() as u32 * (self.current_level() + 1);
    }

    fn advance(&mut self) {
        self.clear_robots();
        for p in &mut self.robots {
            let new = Robots::towards(*p, self.position);
            *p = new;
        }
        for p in &self.robots {
            // place_robot copy to fight borrow checker :(
            if self.board[*p].is_empty() {
                self.board[*p] = Cell::Filled(CellData::Robot);
            } else {
                self.board[*p] = Cell::Filled(CellData::ScrapHeap);
            }
        }
        self.points += self.validate_robots() as u32 * (self.current_level() + 1);
        self.check_safety();
        if self.robots.len() < self.config.robots as usize {
            let p = self.random_border_spot();
            self.robots.push(p);
            self.place_robot(p);
        }
    }
}

impl<'a> Game for Robots<'a> {
    type CellData = CellData;
    fn current_level(&self) -> u32 {
        MAX_LEVEL - self.speed
    }
    fn score(&self) -> u32 {
        self.points
    }
    fn top_score(&self) -> u32 {
        self.score_table.get_top_score(&self.config)
    }
    fn board(&self) -> &Imprint<CellData> {
        &self.board
    }
    fn next(&self) -> Option<&Imprint<CellData>> {
        None
    }
    fn tick(&mut self) -> TickResult {
        match self.status {
            Status::Teleporting(p) => {
                let (x1, y1) = p;
                let (x0, y0) = self.position;
                if self.anim_tick == 0 {
                    self.obliterate(
                        if x0 == 0 { x0 } else { x0 - 1 },
                        if y0 == 0 { y0 } else { y0 - 1 },
                        if x0 == WIDTH - 1 { x0 } else { x0 + 1 },
                        if y0 == HEIGHT - 1 { y0 } else { y0 + 1 },
                    );
                    self.obliterate(
                        if x1 == 1 { x1 } else { x1 - 1 },
                        if y1 == 1 { y1 } else { y1 - 1 },
                        if x1 == WIDTH - 1 { x1 } else { x1 + 1 },
                        if y1 == HEIGHT - 1 { y1 } else { y1 + 1 },
                    );
                    self.position = p;
                    self.status = Status::Active;
                } else {
                    self.anim_tick = (self.anim_tick + 1) % 8;
                }
            }
            Status::Active => {
                if self.input.escape {
                    self.status = Status::Paused;
                    self.input.escape = false;
                } else {
                    if self.input.left {
                        self.input.left = false;
                        self.position = Robots::towards(self.position, (0, self.position.1));
                        self.check_safety();
                    }
                    if self.input.right {
                        self.input.right = false;
                        self.position =
                            Robots::towards(self.position, (WIDTH - 1, self.position.1));
                        self.check_safety();
                    }
                    if self.input.up {
                        self.input.up = false;
                        self.position = Robots::towards(self.position, (self.position.0, 0));
                        self.check_safety();
                    }
                    if self.input.down {
                        self.input.down = false;
                        self.position =
                            Robots::towards(self.position, (self.position.0, HEIGHT - 1));
                        self.check_safety();
                    }
                    if self.input.button_a || self.input.button_b {
                        if self.teleports > 0 {
                            self.teleports -= 1;
                            self.status = Status::Teleporting(self.random_free_spot());
                            self.anim_tick = 0;
                            self.input.button_a = false;
                        }
                    }
                    if self.movement_tick == 0 {
                        self.advance();
                    }
                    self.movement_tick = (self.movement_tick + 1) % self.speed;
                    self.anim_tick = (self.anim_tick + 1) % 4;
                }
            }
            Status::Paused => {
                if self.input.drop || self.input.button_a || self.input.button_b ||
                    self.input.left || self.input.right || self.input.down ||
                    self.input.up
                {
                    self.status = Status::Active;
                } else if self.input.escape {
                    self.input.escape = false;
                    self.status = Status::Raising(0);
                }
            }
            Status::Raising(f) => {
                if f == HEIGHT {
                    self.new_game();
                    self.status = Status::Lowering(HEIGHT);
                } else {
                    self.status = Status::Raising(f + 1);
                }
            }
            Status::Lowering(f) => {
                if f == 0 {
                    self.status = Status::Menu(0);
                } else {
                    self.status = Status::Lowering(f - 1);
                }
            }
            Status::Menu(f) => {
                self.status = Status::Menu((f + 1) % 70);
                if self.input.escape {
                    self.input.escape = false;
                    return TickResult::Exit;
                }
                if self.input.next {
                    self.input.escape = false;
                    return TickResult::NextGame;
                }
                if self.input.prev {
                    self.input.escape = false;
                    return TickResult::PrevGame;
                }
                if self.input.right {
                    self.input.right = false;
                    if self.config.robots < MAX_ROBOTS {
                        self.config.robots += 1;
                        self.new_game();
                    }
                }
                if self.input.left {
                    self.input.left = false;
                    if self.config.robots > 1 {
                        self.config.robots -= 1;
                        self.new_game();
                    }
                }
                if self.input.drop {
                    self.input.drop = false;
                    self.status = Status::Active;
                }
                if self.input.up {
                    self.input.up = false;
                    if self.config.level < MAX_LEVEL - 1 {
                        self.config.level += 1;
                        self.speed = MAX_LEVEL - self.config.level;
                    }
                }
                if self.input.down {
                    self.input.down = false;
                    if self.config.level > 0 {
                        self.config.level -= 1;
                        self.speed = MAX_LEVEL - self.config.level;
                    }
                }
            }
        }
        TickResult::Continue
    }
    fn is_paused(&self) -> bool {
        return self.status == Status::Paused;
    }

    fn input_state(&mut self) -> &mut InputState {
        &mut self.input
    }
}
