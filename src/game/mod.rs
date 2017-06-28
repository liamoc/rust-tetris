use imprint::Imprint;

pub mod robots;
pub mod tetris;
pub mod snake;

pub struct InputState {
    pub escape: bool,
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub button_a: bool,
    pub button_b: bool,
    pub drop: bool,
    pub next: bool,
    pub prev: bool,
    pub skip: u32,
}
impl InputState {
    pub fn new() -> InputState {
        InputState {
            skip: 0,
            escape: false,
            down: false,
            left: false,
            right: false,
            button_a: false,
            button_b: false,
            up: false,
            drop: false,
            next: false,
            prev: false
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TickResult {
    Continue,
    Exit,
    NextGame,
    PrevGame
}

pub trait Game {
    type CellData : Copy;
    fn current_level(&self) -> u32;
    fn score(&self) -> u32;
    fn top_score(&self) -> u32;
    fn board(&self) -> &Imprint<Self::CellData>;
    fn next(&self) -> Option<&Imprint<Self::CellData>>;
    fn tick(&mut self) -> TickResult;
    fn is_paused(&self) -> bool;
    fn input_state(&mut self) -> &mut InputState;
}
