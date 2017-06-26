use game::snake::{Snake, Status};
use game::snake;
use sdl2::render::RenderTarget;
use sdl2::render::Canvas;

use super::*;

pub struct DrawingContext {
    ctx: BaseDrawingContext,
}

impl DrawingContext {
    pub fn new(vp_w: u32, vp_h: u32) -> DrawingContext {
        DrawingContext {
            ctx: BaseDrawingContext::new(vp_w, vp_h, snake::WIDTH as u32, snake::HEIGHT as u32, 0),
        }
    }
}





impl<'a> GameDrawingContext<Snake<'a>> for DrawingContext {
    fn resize(&mut self, vp_w: u32, vp_h: u32) {
        self.ctx = BaseDrawingContext::new(vp_w, vp_h, snake::WIDTH as u32, snake::HEIGHT as u32, 0)
    }

    fn draw_game<T: RenderTarget>(&self, c: &mut Canvas<T>, g: &Snake) -> Result<(), String> {
        self.ctx.draw(c, g)?;
        let main = &self.ctx.main;
        match g.status {
            Status::Active | Status::Paused => {
                c.set_draw_color(HI_COLOR);
                main.draw_box(
                    c,
                    g.food_position.0 as i32,
                    g.food_position.1 as i32,
                )?;
                main.draw_box(
                    c,
                    g.head_position.0 as i32,
                    g.head_position.1 as i32,
                )?;
            }
            Status::Raising(f) => {
                c.set_draw_color(HI_COLOR);
                let (x1,y1) = (g.head_position.0 as i32 - f as i32, g.head_position.1 as i32 - f as i32);
                let (x2,y2) = (g.head_position.0 as i32 + f as i32, g.head_position.1 as i32 + f as i32);
                main.fill_rect(
                    c,
                    x1,y1,x2,y2
                )?;
            }
            Status::Lowering(f) => {
                c.set_draw_color(HI_COLOR);
                main.fill_boxes(c, main.board_h as i32 - f as i32, main.board_h as i32)?;
            }
            Status::Menu(f) => {
                c.set_draw_color(HI_COLOR);
                let o = (f / 2) as i32 - main.board_w as i32;
                // @@  @ @   @   @  @@  @@
                // @ @   @@ @@@ @@ @ @ @
                // @ @ @ @ @ @ @ @ @@   @
                // @  @@@@@  @@   @ @@@@
                let points = [
                    (16, 3),
                    (16, 4),
                    (17, 2),
                    (17, 4),
                    (17, 5),
                    (18, 2),
                    (18, 3),
                    (18, 5),

                    (19, 5),
                    (20, 3),
                    (20, 5),
                    (21, 2),
                    (21, 4),
                    (22, 2),

                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5),
                    (1, 2),
                    (2, 3),
                    (2, 4),
                    (3, 5),

                    (4, 2),
                    (4, 4),
                    (4, 5),
                    (5, 5),

                    (6, 2),
                    (6, 3),
                    (6, 4),
                    (6, 5),
                    (7, 3),
                    (7, 5),
                    (8, 4),
                    (9, 3),

                    (10, 2),
                    (10, 3),
                    (10, 4),
                    (10, 5),
                    (11, 3),
                    (11, 5),
                    (12, 4),
                    (13, 3),

                    (14, 2),
                    (14, 3),
                    (14, 4),
                    (15, 5),
                ];
                for &(x, y) in points.iter() {
                    main.draw_box(c, x - o, y + 6)?;
                }
            }
        }
        c.present();
        Ok(())
    }
}
