#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate rand;
extern crate byteorder;
extern crate app_dirs;

mod imprint;
mod piece;
mod game;
mod score_table;
mod drawing;

use game::Game;
use drawing::DrawingContext;

use app_dirs::{AppDataType, app_root, AppInfo};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;

const FRAMERATE: u32 = 20;

pub fn main() {
    const APP_INFO: AppInfo = AppInfo {
        name: "Tetris",
        author: "Liam O'Connor",
    };
    let mut path = app_root(AppDataType::UserData, &APP_INFO).unwrap();
    path.push("scores");
    let mut game = Game::new(path.as_path()).unwrap();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut rate_limiter = FPSManager::new();
    rate_limiter.set_framerate(FRAMERATE).unwrap();

    let mut window = video_subsystem
        .window("Tetris", 248, 328)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();
    window.set_minimum_size(128, 168).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut dimensions = (canvas.viewport().width(), canvas.viewport().height());

    let mut ctx = DrawingContext::new(
        dimensions.0,
        dimensions.1,
        game::WIDTH as u32,
        (game::HEIGHT + game::BUFFER) as u32,
        game::BUFFER as u32,
    );
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => game.input.escape = true,
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => game.input.escape = false,
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => game.input.escape = true,
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => game.input.escape = false,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => game.input.left = true,
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    game.input.left = false;
                    game.input.skip = 0;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => game.input.right = true,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    game.input.right = false;
                    game.input.skip = 0;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => game.input.down = true,
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => game.input.down = false,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => game.input.rotate_r = true,
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => game.input.rotate_r = false,
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => game.input.rotate_l = true,
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => game.input.rotate_l = false,
                Event::KeyDown { keycode: Some(Keycode::X), .. } => game.input.rotate_r = true,
                Event::KeyUp { keycode: Some(Keycode::X), .. } => game.input.rotate_r = false,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => game.input.drop = true,
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => game.input.drop = false,
                _ => {}
            }
        }
        if !game.tick() {
            break 'running;
        }
        let new_dimensions = (canvas.viewport().width(), canvas.viewport().height());
        if dimensions != new_dimensions {
            dimensions = new_dimensions;
            ctx = DrawingContext::new(
                dimensions.0,
                dimensions.1,
                game::WIDTH as u32,
                (game::HEIGHT + game::BUFFER) as u32,
                game::BUFFER as u32,
            );
        }
        ctx.draw(&mut canvas, &game).unwrap();
        rate_limiter.delay();
    }
}
