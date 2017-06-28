mod numeric;
mod label;
mod board;

pub mod tetris;
pub mod snake;
pub mod robots;

use self::board::BoardDrawingContext;
use self::label::LabelDrawingContext;
use self::numeric::NumericDrawingContext;

use game::{Game};


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

pub struct BaseDrawingContext {
    main: BoardDrawingContext,
    next: BoardDrawingContext,
    labels: LabelDrawingContext,
    points: NumericDrawingContext,
    top: NumericDrawingContext,
    level: NumericDrawingContext,
    border: Rect,
    label_positions: (i32, i32, i32),
}

pub trait GameDrawingContext<G : Game> {
    fn draw_game<T: RenderTarget>(&self, c: &mut Canvas<T>, g: &G) -> Result<(), String>;
    fn resize(&mut self, vp_w: u32, vp_h: u32);
}


impl BaseDrawingContext {
    pub fn new(vp_w: u32, vp_h: u32, board_w: u32, board_h: u32, board_b: u32) -> Self {
        const PADDING_X: u32 = 2;
        const PADDING_Y: u32 = 2;
        let sidebar_w: u32 = (vp_w - PADDING_X * 4) / 3;
        let box_w = (vp_w - PADDING_X * 4 - sidebar_w) / board_w;
        let box_h = (vp_h - PADDING_Y * 4) / (board_h - board_b);
        let sidebar_x = vp_w - sidebar_w + box_w / 2 - PADDING_X;
        BaseDrawingContext {
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

    pub fn draw<T: RenderTarget, G: Game>(
        &self,
        c: &mut Canvas<T>,
        g: &G,
    ) -> Result<(), String> {
        c.set_draw_color(RM_COLOR);
        c.clear();
        c.set_draw_color(BG_COLOR);
        self.main.fill_all_boxes(c)?;
        self.points.draw_bg(c)?;
        self.top.draw_bg(c)?;
        self.level.draw_bg(c)?;
        self.next.fill_boxes(c, 1, 3)?;
        c.set_draw_color(FG_COLOR);
        c.draw_rect(self.border)?;
        // self.labels.draw(c, "NEXT", 0, 2)?;
        self.labels.draw(c, "POINTS", 0, self.label_positions.0)?;
        self.labels.draw(c, "TOP", 0, self.label_positions.1)?;
        self.labels.draw(c, "LEVEL", 0, self.label_positions.2)?;
        self.points.draw_num(c, g.score())?;
        self.top.draw_num(c, g.top_score())?;
        self.level.draw_num(c, g.current_level() + 1)?;
        match g.next() {
            Some(n) => self.next.draw_imprint(c, n, 0, 0)?,
            None => {}
        }
        c.set_draw_color(if g.is_paused() { HI_COLOR } else { FG_COLOR });
        self.main.draw_imprint(c, &g.board(), 0, 0)?;
        Ok(())
    }
}

