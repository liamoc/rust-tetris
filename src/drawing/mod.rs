mod numeric;
mod label;
mod board;

use self::board::BoardDrawingContext;
use self::label::LabelDrawingContext;
use self::numeric::NumericDrawingContext;

use ::game::{Game, Status};

use sdl2::render::RenderTarget;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub const RM_COLOR: Color = Color {
    r: 158,
    g: 173,
    b: 134,
    a: 255,
};
pub const BG_COLOR: Color = Color {
    r: 135,
    g: 147,
    b: 114,
    a: 255,
};

pub const FG_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const HI_COLOR: Color = Color {
    r: 64,
    g: 0,
    b: 0,
    a: 255,
};


pub struct DrawingContext {
    main: BoardDrawingContext,
    next: BoardDrawingContext,
    labels: LabelDrawingContext,
    points: NumericDrawingContext,
    top: NumericDrawingContext,
    level: NumericDrawingContext,
    border: Rect,
    label_positions: (i32, i32, i32),
}
impl DrawingContext {
    pub fn new(vp_w: u32, vp_h: u32, board_w: u32, board_h: u32, board_b: u32) -> DrawingContext {
        const PADDING_X: u32 = 2;
        const PADDING_Y: u32 = 2;
        let sidebar_w: u32 = (vp_w - PADDING_X * 4) / 3;
        let box_w = (vp_w - PADDING_X * 4 - sidebar_w) / board_w;
        let box_h = (vp_h - PADDING_Y * 4) / (board_h - board_b);
        let sidebar_x = vp_w - sidebar_w + box_w / 2 - PADDING_X;
        DrawingContext {
            main: BoardDrawingContext {
                offset_x: PADDING_X as u32 + 1,
                offset_y: PADDING_Y as u32 + 1,
                box_w: box_w,
                box_h: box_h,
                board_w: board_w,
                board_h: board_h,
                buffer_h: board_b,
            },
            next: BoardDrawingContext {
                offset_x: sidebar_x,
                offset_y: PADDING_Y as u32 + 1 + box_h,
                box_w: box_w,
                box_h: box_h,
                board_w: 4,
                board_h: 3,
                buffer_h: 1,
            },
            labels: LabelDrawingContext {
                w: box_w as i32 * 2 / 5,
                h: box_h as i32 * 2 / 3,
                spacing: box_w as i32 / 4,
                offset_x: sidebar_x as i32,
                offset_y: box_h as i32 / 4,
            },
            level: NumericDrawingContext {
                x: sidebar_x as i32 + (box_w as i32 / 2 + box_w as i32 / 4) * 4,
                y: box_h as i32 / 4 + box_h as i32 * 13,
                max_digits: 2,
                cell_w: box_w as i32 / 2,
                h: box_h as i32,
                spacing: box_w as i32 / 4,
            },
            top: NumericDrawingContext {
                x: sidebar_x as i32,
                y: box_h as i32 / 4 + box_h as i32 * 9,
                max_digits: 6,
                cell_w: box_w as i32 / 2,
                h: box_h as i32,
                spacing: box_w as i32 / 4,
            },
            points: NumericDrawingContext {
                x: sidebar_x as i32,
                y: box_h as i32 / 4 + box_h as i32 * 5,
                max_digits: 6,
                cell_w: box_w as i32 / 2,
                h: box_h as i32,
                spacing: box_w as i32 / 4,
            },
            border: Rect::new(
                PADDING_X as i32,
                PADDING_Y as i32,
                box_w * board_w as u32 + PADDING_X + 1,
                box_h * (board_h - board_b) as u32 + PADDING_Y + 1,
            ),
            label_positions: (box_h as i32 * 4, box_h as i32 * 8, box_h as i32 * 12),
        }
    }

    pub fn draw<T: RenderTarget>(&self, c: &mut Canvas<T>, g: &Game) -> Result<(), String> {
        c.set_draw_color(RM_COLOR);
        c.clear();
        c.set_draw_color(FG_COLOR);
        c.draw_rect(self.border)?;
        c.set_draw_color(BG_COLOR);
        self.main.fill_all_boxes(c)?;
        self.labels.draw(c, "NEXT", 0, 2)?;
        self.labels.draw(c, "POINTS", 0, self.label_positions.0)?;
        self.labels.draw(c, "TOP", 0, self.label_positions.1)?;
        self.labels.draw(c, "LEVEL", 0, self.label_positions.2)?;
        c.set_draw_color(BG_COLOR);
        self.points.draw_bg(c)?;
        c.set_draw_color(FG_COLOR);
        self.points.draw_num(c, g.points)?;
        c.set_draw_color(BG_COLOR);
        self.top.draw_bg(c)?;
        c.set_draw_color(FG_COLOR);
        self.top.draw_num(c, g.score_table.get_top_score(&g.config))?;
        c.set_draw_color(BG_COLOR);
        self.level.draw_bg(c)?;
        c.set_draw_color(FG_COLOR);
        self.level.draw_num(c, g.current_level() + 1)?;
        c.set_draw_color(BG_COLOR);
        self.next.fill_boxes(c, 1, 3)?;
        c.set_draw_color(if g.status == Status::Paused {
            HI_COLOR
        } else {
            FG_COLOR
        });
        self.main.draw_imprint(c, &g.board, 0, 0)?;
        match g.status {
            Status::Active | Status::Paused => {
                self.main.draw_imprint(
                    c,
                    &g.current.imprint(),
                    g.position.0,
                    g.position.1,
                )?;
                c.set_draw_color(FG_COLOR);
                self.next.draw_imprint(c, &g.next.imprint(), 0, 0)?
            }
            Status::Raising(f) => {
                c.set_draw_color(HI_COLOR);
                self.main.draw_imprint(
                    c,
                    &g.current.imprint(),
                    g.position.0,
                    g.position.1,
                )?;
                c.set_draw_color(HI_COLOR);
                self.main.fill_boxes(c, f as i32, self.main.board_h as i32)?;
            }
            Status::Lowering(f) => {
                c.set_draw_color(HI_COLOR);
                self.main.fill_boxes(c, f as i32, self.main.board_h as i32)?;
            }
            Status::Menu(f) => {
                c.set_draw_color(HI_COLOR);
                let o = (f / 2) as i32 - self.main.board_w as i32;
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
                    self.main.draw_box(c, x - o, y)?;
                }
            }

            Status::Clearing(f) => {
                c.set_draw_color(if f % 2 == 0 { HI_COLOR } else { BG_COLOR });
                for y in &g.lines {
                    self.main.fill_boxes(c, *y as i32, *y as i32 + 1)?;
                }
                c.set_draw_color(FG_COLOR);
                self.next.draw_imprint(c, &g.next.imprint(), 0, 0)?
            }

            Status::Placing(p, x, y) => {
                c.set_draw_color(HI_COLOR);
                self.main.draw_imprint(c, &p.imprint(), x, y)?;
                c.set_draw_color(FG_COLOR);
                self.next.draw_imprint(c, &g.next.imprint(), 0, 0)?
            }
        }
        c.present();
        Ok(())
    }
}
