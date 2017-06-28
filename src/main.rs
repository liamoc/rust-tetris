#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate rand;
extern crate byteorder;
extern crate app_dirs;

mod imprint;
mod game;
mod drawing;

use game::tetris::Tetris;
use game::snake::Snake;
use game::robots::Robots;
use drawing::{GameDrawingContext, tetris, snake, robots};
use game::{Game, TickResult};

use app_dirs::{AppDataType, app_root, AppInfo};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;

const FRAMERATE: u32 = 20;

use sdl2::render::{RenderTarget, Canvas};
use sdl2::EventPump;

pub fn game_loop<G: Game, C: GameDrawingContext<G>, T: RenderTarget>(
    game: &mut G,
    ctx: &mut C,
    canvas: &mut Canvas<T>,
    event_pump: &mut EventPump,
) -> TickResult {
    let mut rate_limiter = FPSManager::new();
    rate_limiter.set_framerate(FRAMERATE).unwrap();
    let mut dimensions = (canvas.viewport().width(), canvas.viewport().height());
    loop {
        for event in event_pump.poll_iter() {
            let input = game.input_state();
            match event {
                Event::Quit { .. } => {
                    return TickResult::Exit;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => input.escape = true,
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => input.escape = false,
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => input.escape = true,
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => input.escape = false,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => input.left = true,
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    input.left = false;
                    input.skip = 0;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => input.right = true,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    input.right = false;
                    input.skip = 0;
                }
                Event::KeyDown { keycode: Some(Keycode::RightBracket), .. } => input.next = true,
                Event::KeyUp { keycode: Some(Keycode::RightBracket), .. } => input.next = false,
                Event::KeyDown { keycode: Some(Keycode::LeftBracket), .. } => input.prev = true,
                Event::KeyUp { keycode: Some(Keycode::LeftBracket), .. } => input.prev = false,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => input.down = true,
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => input.down = false,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => input.up = true,
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => input.up = false,
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => input.button_a = true,
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => input.button_a = false,
                Event::KeyDown { keycode: Some(Keycode::X), .. } => input.button_b = true,
                Event::KeyUp { keycode: Some(Keycode::X), .. } => input.button_b = false,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => input.drop = true,
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => input.drop = false,
                _ => {}
            }
        }
        match game.tick() {
            TickResult::Continue => {}
            x => return x,
        }
        let new_dimensions = (canvas.viewport().width(), canvas.viewport().height());
        if dimensions != new_dimensions {
            dimensions = new_dimensions;
            ctx.resize(dimensions.0, dimensions.1);
        }
        ctx.draw_game(canvas, &game).unwrap();
        rate_limiter.delay();
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum GameTag {
    Tetris,
    Snake,
    Robots
}

static GAME_TAGS: [GameTag; 3] = [GameTag::Tetris, GameTag::Snake, GameTag::Robots];

const APP_INFO: AppInfo = AppInfo {
    name: "Tetris",
    author: "Liam O'Connor",
};


fn play_game<T: RenderTarget>(
    g: GameTag,
    canvas: &mut Canvas<T>,
    event_pump: &mut EventPump,
) -> TickResult {
    let mut path = app_root(AppDataType::UserData, &APP_INFO).unwrap();
    let dimensions = (canvas.viewport().width(), canvas.viewport().height());
    match g {
        GameTag::Tetris => {
            path.push("tetris");
            let mut game = Tetris::new(path.as_path()).unwrap();
            let mut ctx = tetris::DrawingContext::new(dimensions.0, dimensions.1);
            game_loop(&mut game, &mut ctx, canvas, event_pump)
        }
        GameTag::Snake => {
            path.push("snake");
            let mut game = Snake::new(path.as_path()).unwrap();
            let mut ctx = snake::DrawingContext::new(dimensions.0, dimensions.1);
            game_loop(&mut game, &mut ctx, canvas, event_pump)
        }
        GameTag::Robots => {
            path.push("robots");
            let mut game = Robots::new(path.as_path()).unwrap();
            let mut ctx = robots::DrawingContext::new(dimensions.0, dimensions.1);
            game_loop(&mut game, &mut ctx, canvas, event_pump)
        }
    }
}


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("Brick Games", 248, 328)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();
    window.set_minimum_size(128, 168).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut current_game: usize = 0;
    loop {
        match play_game(GAME_TAGS[current_game], &mut canvas, &mut event_pump) {
            TickResult::NextGame => current_game = (current_game + 1) % GAME_TAGS.len(),
            TickResult::PrevGame => {
                if current_game == 0 {
                    current_game = GAME_TAGS.len() - 1
                } else {
                    current_game = current_game - 1
                }
            }
            _ => break,
        }
    }

}
