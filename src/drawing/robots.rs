use game::robots::{Robots, Status};
use game::robots;
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
                robots::WIDTH as u32,
                robots::HEIGHT as u32,
                0,
            ),
        }
    }
}





impl<'a> GameDrawingContext<Robots<'a>> for DrawingContext {
    fn resize(&mut self, vp_w: u32, vp_h: u32) {
        self.ctx =
            BaseDrawingContext::new(vp_w, vp_h, robots::WIDTH as u32, robots::HEIGHT as u32, 0)
    }

    fn draw_game<T: RenderTarget>(&self, c: &mut Canvas<T>, g: &Robots) -> Result<(), String> {
        self.ctx.draw(c, g)?;
        let main = &self.ctx.main;
        match g.status {
            Status::Teleporting(p) => {
                c.set_draw_color(if (g.anim_tick % 4) / 2 == 0 {
                    HI_COLOR
                } else {
                    FG_COLOR
                });
                for &(x, y) in &g.robots {
                    main.draw_box(c, x as i32, y as i32)?;
                }
                c.set_draw_color(HI_COLOR);
                if g.anim_tick <= 6 && g.anim_tick > 0 {
                    main.fill_rect(
                        c,
                        g.position.0 as i32 - 1,
                        g.position.1 as i32 - 1,
                        g.position.0 as i32 + 2,
                        g.position.1 as i32 + 2,
                    )?;
                }
                if g.anim_tick >= 4 || g.anim_tick == 0 {
                    main.fill_rect(
                        c,
                        p.0 as i32 - 1,
                        p.1 as i32 - 1,
                        p.0 as i32 + 2,
                        p.1 as i32 + 2,
                    )?;
                }
                c.set_draw_color(FG_COLOR);
                for y in 0..2i32 {
                    for x in 0..4i32 {
                        if g.teleports as i32 > (y * 4 + x) {
                            if g.teleports as i32 == (y * 4 + x + 1) {
                                c.set_draw_color(HI_COLOR);
                            }
                            self.ctx.next.draw_box(c,x,y+1)?;
                        }
                    }
                }
            }
            Status::Active | Status::Paused => {
                c.set_draw_color(if (g.anim_tick % 4) / 2 == 0 {
                    HI_COLOR
                } else {
                    FG_COLOR
                });
                for &(x, y) in &g.robots {
                    main.draw_box(c, x as i32, y as i32)?;
                }
                c.set_draw_color(HI_COLOR);
                main.draw_box(c, g.position.0 as i32, g.position.1 as i32)?;
                c.set_draw_color(FG_COLOR);
                for y in 0..2i32 {
                    for x in 0..4i32 {
                        if g.teleports as i32 > (y * 4 + x) {
                            self.ctx.next.draw_box(c,x,y+1)?;
                        }
                    }
                }
            }
            Status::Raising(f) => {
                c.set_draw_color(HI_COLOR);
                let (x1, y1) = (
                    g.position.0 as i32 - f as i32,
                    g.position.1 as i32 - f as i32,
                );
                let (x2, y2) = (
                    g.position.0 as i32 + f as i32,
                    g.position.1 as i32 + f as i32,
                );
                main.fill_rect(c, x1, y1, x2, y2)?;
            }
            Status::Lowering(f) => {
                c.set_draw_color(HI_COLOR);
                main.fill_boxes(
                    c,
                    main.board_h as i32 - f as i32,
                    main.board_h as i32,
                )?;
            }
            Status::Menu(f) => {
                c.set_draw_color(HI_COLOR);
                let o = (f / 2) as i32 - main.board_w as i32;
                // @@      @       @
                // @ @  @  @@   @  @@ @
                // @@  @ @ @ @ @ @ @   @
                // @ @  @  @@   @  @@ @@
                let points = [
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (1, 0),
                    (1, 2),
                    (2, 1),
                    (2, 3),
                    (4, 2),
                    (5, 1),
                    (5, 3),
                    (6, 2),
                    (8, 0),
                    (8, 1),
                    (8, 2),
                    (8, 3),
                    (9, 1),
                    (9, 3),
                    (10, 2),
                    (12, 2),
                    (13, 1),
                    (13, 3),
                    (14, 2),
                    (16, 0),
                    (16, 1),
                    (16, 2),
                    (16, 3),
                    (17, 1),
                    (17, 3),
                    (19, 1),
                    (19, 3),
                    (20, 2),
                    (20, 3),
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
