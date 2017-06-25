#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate rand;
extern crate byteorder;
extern crate app_dirs;

mod imprint;
mod game;
mod drawing;

use game::tetris::{Tetris};
use drawing::tetris;
use drawing::GameDrawingContext;
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
) {
    let mut rate_limiter = FPSManager::new();
    rate_limiter.set_framerate(FRAMERATE).unwrap();
    let mut dimensions = (canvas.viewport().width(), canvas.viewport().height());
    'running: loop {
        for event in event_pump.poll_iter() {
            let input = game.input_state();
            match event {
                Event::Quit { .. } => break 'running,
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
        if game.tick() != TickResult::Continue {
            break 'running;
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



pub fn main() {
    const APP_INFO: AppInfo = AppInfo {
        name: "Tetris",
        author: "Liam O'Connor",
    };
    let mut path = app_root(AppDataType::UserData, &APP_INFO).unwrap();
    path.push("tetris");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("Tetris", 248, 328)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();
    window.set_minimum_size(128, 168).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let dimensions = (canvas.viewport().width(), canvas.viewport().height());

    let mut game = Tetris::new(path.as_path()).unwrap();
    let mut ctx = tetris::DrawingContext::new(dimensions.0, dimensions.1);
    game_loop(&mut game, &mut ctx, &mut canvas, &mut event_pump);
}
