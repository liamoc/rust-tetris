use std::path::Path;

mod score_table;
mod fields;

use self::score_table::ScoreTable;

use game::{Game, InputState, TickResult};

use imprint::{Imprint, Cell};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub const MAX_LEVEL: u32 = ::FRAMERATE; // should always be <= FRAMERATE

pub struct Config {
    pub field: u32,
    pub level: u32,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Paused,
    Raising(usize),
    Lowering(usize),
    Menu(u32),
}

const BONUS_TIME : u32 = 24;
const NO_BONUS_TIME : u32 = 32;

pub struct Snake<'a> {
    pub config: Config,
    pub status: Status,
    pub head_position: (usize, usize),
    pub direction: Direction,
    pub food_position: (usize,usize),
    pub bonus_position: Option<(usize, usize)>,
    pub anim_tick: u32,
    movement_tick: u32,
    tail_position: (usize,usize),
    input: InputState,
    board: Imprint<CellData>,
    points: u32,
    score_table: ScoreTable<'a>,
    speed: u32,
    growth: u32,
    pub bonus_timer: u32,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    fn turn_left(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }
    fn turn_right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellData {
    Snake(Direction),
    Wall
}

fn move_dir((x,y) : (usize,usize), d : Direction) -> (usize,usize) {
    let rx = match d {
        Direction::Up | Direction::Down => x,
        Direction::Left => if x == 0 { WIDTH - 1 } else { x - 1 },
        Direction::Right => (x + 1) % WIDTH ,
    };
    let ry = match d {
        Direction::Left | Direction::Right => y,
        Direction::Up => if y == 0 { HEIGHT - 1 } else { y - 1 },
        Direction::Down => (y + 1) % HEIGHT ,
    };
    (rx,ry)
}
impl<'a> Snake<'a> {

    pub fn new(filename: &'a Path) -> ::std::io::Result<Self> {
        let mut g = Snake {
            config: Config { field: 0, level: 9 },
            status: Status::Menu(0),
            board: fields::field(0),
            movement_tick: 0,
            speed: MAX_LEVEL - 9,
            direction: Direction::Right,
            head_position: (WIDTH/2, HEIGHT/2),
            tail_position: (WIDTH/2, HEIGHT/2),
            food_position: (0, 0),
            bonus_position: None,
            points: 0,
            score_table: ScoreTable::new(filename)?,
            input: InputState::new(),
            growth: 3,
            bonus_timer: NO_BONUS_TIME,
            anim_tick:0,
        };
        g.food_position = g.random_free_spot();
        Ok(g)
    }

    fn new_game(&mut self) {
        self.board = fields::field(self.config.field as usize);
        self.score_table
            .update_scores(&self.config, self.points)
            .unwrap();
        self.direction = Direction::Right;
        self.tail_position = (WIDTH/2,HEIGHT/2);
        self.head_position = (WIDTH/2,HEIGHT/2);
        self.bonus_position = None;
        self.growth = 3;
        self.food_position = self.random_free_spot();
        self.bonus_timer = NO_BONUS_TIME;
        self.points = 0;
        self.speed = MAX_LEVEL - self.config.level;
    }

    fn random_free_spot(&self) -> (usize,usize){
        let x = ::rand::random::<u32>() as usize % WIDTH;
        let y = ::rand::random::<u32>() as usize % HEIGHT;
        if !self.board[(x,y)].is_empty() || self.head_position == (x,y) || self.food_position == (x,y) {
            for xo in 0..WIDTH {
                for yo in 0..HEIGHT {
                    let new = ((x + xo) % WIDTH, (y + yo) % HEIGHT);
                    if self.board[new].is_empty() && new != self.head_position && new != self.food_position {
                        return new;
                    }
                }
            }
        }
        return (x,y)
    }

    fn advance(&mut self) {
        self.bonus_timer -= 1;
        if self.bonus_timer == 0 {
            match self.bonus_position {
                Some(_) => {
                    self.bonus_position = None;
                    self.bonus_timer = NO_BONUS_TIME;
                }
                None => {
                    self.bonus_position = Some(self.random_free_spot());
                    self.bonus_timer = BONUS_TIME;
                }
            }
        }
        self.board[self.head_position] = Cell::Filled(CellData::Snake(self.direction));
        let new_loc = move_dir(self.head_position, self.direction);
        if !self.board[new_loc].is_empty() {
            self.status = Status::Raising(0);
            return;
        }
        if new_loc == self.food_position {
            self.points += MAX_LEVEL - self.speed + 1;
            self.growth += 1;
            self.food_position = self.random_free_spot();
        }
        self.head_position = new_loc;
        self.board[self.head_position] = Cell::Filled(CellData::Snake(self.direction));
        if Some(self.head_position) == self.bonus_position {
            self.points += (self.bonus_timer * 4) + 4;
            self.bonus_position = None;
            self.bonus_timer = NO_BONUS_TIME;
        }
        if self.growth > 0 {
            self.growth -= 1;
        } else {
            if let Cell::Filled(CellData::Snake(d)) = self.board[self.tail_position] {
                self.board[self.tail_position] = Cell::Empty;
                self.tail_position = move_dir(self.tail_position, d);
            }
        }
    }

    fn set_direction(&mut self, d1 : Direction) {
        match self.board[self.head_position] {
            Cell::Filled(CellData::Snake(d2)) if d1 == d2.turn_right().turn_right() => return,
            _ => {}
        }
        self.direction = d1;
    }
}

impl<'a> Game for Snake<'a> {
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
            Status::Active => {
                if self.input.escape {
                    self.status = Status::Paused;
                    self.input.escape = false;
                } else {
                    if self.input.left {
                        self.set_direction(Direction::Left);
                    }
                    if self.input.right {
                        self.set_direction(Direction::Right);
                    }
                    if self.input.up {
                        self.set_direction(Direction::Up);
                    }
                    if self.input.down {
                        self.set_direction(Direction::Down);
                    }
                    if self.input.button_a {
                        let d = self.direction.turn_left();
                        self.set_direction(d);
                    }
                    if self.input.button_b {
                        let d = self.direction.turn_right();
                        self.set_direction(d);
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
                    self.input.left || self.input.right || self.input.down || self.input.up
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
                    self.config.field = (self.config.field + 1) % fields::MAX_FIELDS as u32;
                    self.new_game();
                }
                if self.input.left {
                    self.input.left= false;
                    if self.config.field == 0 {
                        self.config.field = fields::MAX_FIELDS as u32 - 1;
                    } else {
                        self.config.field -= 1;
                    }
                    self.new_game();
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
