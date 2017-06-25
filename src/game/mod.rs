use imprint::Imprint;

pub mod tetris;

pub struct InputState {
    pub escape: bool,
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub button_a: bool,
    pub button_b: bool,
    pub drop: bool,
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
    fn current_level(&self) -> u32;
    fn score(&self) -> u32;
    fn top_score(&self) -> u32;
    fn board(&self) -> &Imprint;
    fn next(&self) -> Option<&Imprint>;
    fn tick(&mut self) -> TickResult;
    fn is_paused(&self) -> bool;
    fn input_state(&mut self) -> &mut InputState;
}
