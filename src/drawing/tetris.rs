use game::tetris::{Tetris, Status};
use game::tetris;
use sdl2::render::RenderTarget;
use sdl2::render::Canvas;

use super::*;

pub struct DrawingContext {
    ctx: BaseDrawingContext,
}

impl DrawingContext {
    pub fn new(vp_w: u32, vp_h: u32) -> DrawingContext {
        DrawingContext {
            ctx: BaseDrawingContext::new(
                vp_w,
                vp_h,
                tetris::WIDTH as u32,
                (tetris::HEIGHT + tetris::BUFFER) as u32,
                tetris::BUFFER as u32,
            ),
        }
    }
}


impl<'a> GameDrawingContext<Tetris<'a>> for DrawingContext {
    fn resize(&mut self, vp_w : u32, vp_h : u32) {
        self.ctx = BaseDrawingContext::new(
            vp_w,
            vp_h,
            tetris::WIDTH as u32,
            (tetris::HEIGHT + tetris::BUFFER) as u32,
            tetris::BUFFER as u32,
        )
    }
    fn draw_game<T: RenderTarget>(&self, c: &mut Canvas<T>, g: &Tetris) -> Result<(), String> {
        self.ctx.draw(c, g)?;
        let main = &self.ctx.main;
        match g.status {
            Status::Active | Status::Paused => {
                main.draw_imprint(
                    c,
                    &g.current.imprint(),
                    g.position.0,
                    g.position.1,
                )?;
            }
            Status::Raising(f) => {
                c.set_draw_color(HI_COLOR);
                main.draw_imprint(
                    c,
                    &g.current.imprint(),
                    g.position.0,
                    g.position.1,
                )?;
                c.set_draw_color(HI_COLOR);
                main.fill_boxes(c, f as i32, main.board_h as i32)?;
            }
            Status::Lowering(f) => {
                c.set_draw_color(HI_COLOR);
                main.fill_boxes(c, f as i32, main.board_h as i32)?;
            }
            Status::Menu(f) => {
                c.set_draw_color(HI_COLOR);
                let o = (f / 2) as i32 - main.board_w as i32;
                let points = [
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (1, 4),
                    (1, 5),
                    (1, 6),
                    (2, 2),
                    (8, 2),
                    (9, 2),
                    (9, 3),
                    (9, 4),
                    (9, 5),
                    (9, 6),
                    (10, 2),
                    (4, 2),
                    (4, 3),
                    (4, 4),
                    (4, 5),
                    (4, 6),
                    (5, 2),
                    (6, 2),
                    (5, 4),
                    (5, 6),
                    (6, 6),
                    (12, 2),
                    (13, 2),
                    (14, 2),
                    (12, 3),
                    (12, 4),
                    (12, 5),
                    (12, 6),
                    (13, 4),
                    (14, 5),
                    (14, 3),
                    (14, 6),
                    (16, 2),
                    (16, 6),
                    (17, 2),
                    (17, 3),
                    (17, 4),
                    (17, 5),
                    (17, 6),
                    (18, 2),
                    (18, 6),
                    (20, 2),
                    (20, 3),
                    (21, 2),
                    (21, 4),
                    (22, 2),
                    (22, 5),
                    (22, 6),
                    (22, 4),
                    (20, 4),
                    (21, 6),
                    (20, 6),
                ];
                for &(x, y) in points.iter() {
                    main.draw_box(c, x - o, y)?;
                }
            }

            Status::Clearing(f) => {
                c.set_draw_color(if f % 2 == 0 { HI_COLOR } else { BG_COLOR });
                for y in &g.lines {
                    main.fill_boxes(c, *y as i32, *y as i32 + 1)?;
                }
            }

            Status::Placing(p, x, y) => {
                c.set_draw_color(HI_COLOR);
                main.draw_imprint(c, &p.imprint(), x, y)?;
            }
        }
        c.present();
        Ok(())
    }
}
