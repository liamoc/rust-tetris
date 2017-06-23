#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate rand;
extern crate byteorder;
extern crate app_dirs;


mod cell;
mod imprint;
mod piece;

use cell::Cell;
use imprint::Imprint;
use piece::Piece;

use app_dirs::*;

use std::path::Path;
use std::fs::File;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const BUFFER: usize = 2;
const ADVANCE_SPEED: i32 = 11;
const FRAMERATE: u32 = 20;
const MAX_BTYPE: u32 = 14;
const KEY_DELAY: u32 = 2;
struct InputState {
    escape: bool,
    down: bool,
    left: bool,
    right: bool,
    rotate_l: bool,
    rotate_r: bool,
    drop: bool,
    skip: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Paused,
    Raising(usize),
    Lowering(usize),
    Menu(u32),
    Clearing(i32),
    Placing(Piece, i32, i32),
}

struct BoardDrawingContext {
    offset_x: u32,
    offset_y: u32,
    box_w: u32,
    box_h: u32,
    buffer_h: u32,
    board_w: u32,
    board_h: u32,
}

struct ScoreTable<'a> {
    scores: [[u32; MAX_BTYPE as usize + 1]; FRAMERATE as usize],
    filename: &'a Path,
}

impl<'a> ScoreTable<'a> {
    fn new(filename: &Path) -> std::io::Result<ScoreTable> {
        let mut it = ScoreTable {
            scores: [[0; MAX_BTYPE as usize + 1]; FRAMERATE as usize],
            filename: filename,
        };
        match File::open(it.filename) {
            Ok(mut file) => {
                for i in 0..FRAMERATE as usize {
                    for j in 0..MAX_BTYPE as usize + 1 {
                        it.scores[i][j] = file.read_u32::<LittleEndian>()?;
                    }
                }
            }
            Err(_) => {}
        }
        Ok(it)
    }

    fn save_scores(&self) -> std::io::Result<()> {
        let mut file = File::create(self.filename)?;
        for i in 0..FRAMERATE as usize {
            for j in 0..MAX_BTYPE as usize + 1 {
                file.write_u32::<LittleEndian>(self.scores[i][j])?;
            }
        }
        Ok(())
    }
    fn get_top_score(&self, c: &Config) -> u32 {
        self.scores[c.level as usize][c.btype as usize]
    }

    fn update_scores(&mut self, c: &Config, score: u32) -> std::io::Result<()> {
        if self.scores[c.level as usize][c.btype as usize] < score {
            self.scores[c.level as usize][c.btype as usize] = score;
            self.save_scores()?;
        }
        Ok(())
    }
}

const RM_COLOR: Color = Color {
    r: 158,
    g: 173,
    b: 134,
    a: 255,
};
const BG_COLOR: Color = Color {
    r: 135,
    g: 147,
    b: 114,
    a: 255,
};
const FG_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
const HI_COLOR: Color = Color {
    r: 64,
    g: 0,
    b: 0,
    a: 255,
};
impl BoardDrawingContext {
    fn draw_box<T: RenderTarget>(&self, c: &mut Canvas<T>, px: i32, py: i32) -> Result<(), String> {
        if px < 0 || px >= self.board_w as i32 {
            return Ok(());
        };
        if py < self.buffer_h as i32 || py >= self.board_h as i32 as i32 {
            return Ok(());
        };
        const PADDING_X: u32 = 1;
        const PADDING_Y: u32 = 1;
        let w = self.box_w;
        let h = self.box_h;
        let x = self.offset_x as i32 + px * self.box_w as i32;
        let y = self.offset_y as i32 + (py - self.buffer_h as i32) * self.box_h as i32;
        c.draw_rect(Rect::new(
            x + PADDING_X as i32,
            y + PADDING_Y as i32,
            w - PADDING_X,
            h - PADDING_Y,
        ))?;
        c.draw_rect(Rect::new(
            x + PADDING_X as i32 + 1,
            y + PADDING_Y as i32 + 1,
            w - PADDING_X - 2,
            h - PADDING_Y - 2,
        ))?;
        c.fill_rect(Rect::new(
            x + PADDING_X as i32 * 4,
            y + PADDING_Y as i32 * 4,
            w - PADDING_X * 7,
            h - PADDING_Y * 7,
        ))
    }
    fn fill_boxes<T: RenderTarget>(
        &self,
        c: &mut Canvas<T>,
        y1: i32,
        y2: i32,
    ) -> Result<(), String> {
        for y in y1..y2 {
            for x in 0..self.board_w {
                self.draw_box(c, x as i32, y)?;
            }
        }
        Ok(())
    }
    fn fill_all_boxes<T: RenderTarget>(&self, c: &mut Canvas<T>) -> Result<(), String> {
        self.fill_boxes(c, self.buffer_h as i32, self.board_h as i32)
    }
    fn draw_imprint<T: RenderTarget>(
        &self,
        c: &mut Canvas<T>,
        p: &Imprint,
        x: i32,
        y: i32,
    ) -> Result<(), String> {
        let (w, h) = p.size();
        for cy in 0..h {
            for cx in 0..w {
                if p[(cx, cy)] != Cell::Empty {
                    self.draw_box(c, x + cx as i32, y + cy as i32)?;
                }
            }
        }
        Ok(())
    }
}
struct Digits {
    n: u32,
    i: bool,
}

impl Digits {
    fn new(n: u32) -> Self {
        Digits { n: n, i: n == 0 }
    }
}
struct DigitsBG {
    n: u32,
    y: u32,
}
impl DigitsBG {
    fn new(n: u32, y: u32) -> Self {
        DigitsBG { n: n, y: y }
    }
}

impl Iterator for DigitsBG {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            Some(self.y)
        }
    }
}
impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i {
            self.i = false;
            Some(0)
        } else {
            if self.n == 0 {
                None
            } else {
                let v = Some(self.n % 10);
                self.n /= 10;
                v
            }
        }

    }
}
struct NumericDrawingContext {
    max_digits: u32,
    x: i32,
    y: i32,
    cell_w: i32,
    h: i32,
    spacing: i32,
}
impl NumericDrawingContext {
    fn draw_bg<T: RenderTarget>(&self, c: &mut Canvas<T>) -> Result<(), String> {
        self.draw(c, DigitsBG::new(self.max_digits, 8))
    }
    fn draw_num<T: RenderTarget>(&self, c: &mut Canvas<T>, num: u32) -> Result<(), String> {
        let max = 10u32.pow(self.max_digits) - 1;
        self.draw(c, Digits::new(if num <= max { num } else { max }))
    }
    fn draw<I, T: RenderTarget>(&self, c: &mut Canvas<T>, num: I) -> Result<(), String>
    where
        I: Iterator<Item = u32>,
    {

        fn draw_horiz_segment<T: RenderTarget>(
            c: &mut Canvas<T>,
            x1: i32,
            x2: i32,
            y: i32,
            fat: bool,
        ) -> Result<(), String> {
            c.draw_line((x1 + 1, y), (x2 - 1, y))?;
            if fat {
                c.draw_line((x1 + 2, y - 1), (x2 - 2, y - 1))?;
                c.draw_line((x1 + 2, y + 1), (x2 - 2, y + 1))?;
            }
            Ok(())
        }
        fn draw_vert_segment<T: RenderTarget>(
            c: &mut Canvas<T>,
            x: i32,
            y1: i32,
            y2: i32,
            fat: bool,
        ) -> Result<(), String> {
            c.draw_line((x, y1 + 1), (x, y2 - 1))?;
            if fat {
                c.draw_line((x + 1, y1 + 2), (x + 1, y2 - 2))?;
                c.draw_line((x - 1, y1 + 2), (x - 1, y2 - 2))?;
            }
            Ok(())
        }

        let y = self.y;
        let w = self.cell_w;
        let h = self.h;
        let s = self.spacing;
        let mut x = self.x + (w + s) * self.max_digits as i32;
        let fat = h > 12 && w > 6;
        for i in num {
            x -= w + s;
            match i {
                9 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                8 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                7 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                6 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                5 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                    draw_vert_segment(c, x, y, y + h / 2, fat)?;
                }
                4 => {
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                3 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                2 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                }
                1 => {
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;

                }
                0 => {
                    draw_horiz_segment(c, x, x + w, y, fat)?;
                    draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;

                }
                _ => {}
            }
        }
        Ok(())
    }
}
struct LabelDrawingContext {
    w: i32,
    h: i32,
    spacing: i32,
    offset_x: i32,
    offset_y: i32,
}

impl LabelDrawingContext {
    fn draw<T: RenderTarget>(
        &self,
        c: &mut Canvas<T>,
        txt: &str,
        x0: i32,
        y0: i32,
    ) -> Result<(), String> {
        let mut x = x0 + self.offset_x;
        let y = y0 + self.offset_y;
        let w = self.w;
        let h = self.h;
        let s = self.spacing;
        for i in txt.chars() {
            match i {
                'N' => {
                    c.draw_line((x, y + h), (x, y))?;
                    c.draw_line((x, y), (x + w, y + h))?;
                    c.draw_line((x + w, y + h), (x + w, y))?;
                }
                'E' => {
                    c.draw_line((x, y), (x, y + h))?;
                    c.draw_line((x, y), (x + w, y))?;
                    c.draw_line((x, y + h / 2), (x + (w * 2 / 3), y + h / 2))?;
                    c.draw_line((x, y + h), (x + w, y + h))?;
                }
                'X' => {
                    c.draw_line((x, y), (x + w, y + h))?;
                    c.draw_line((x, y + h), (x + w, y))?;
                }
                'T' => {
                    c.draw_line((x, y), (x + w, y))?;
                    c.draw_line((x + w / 2, y), (x + w / 2, y + h))?;
                }
                'P' => {
                    c.draw_line((x, y), (x + w - 1, y))?;
                    c.draw_line((x, y), (x, y + h))?;
                    c.draw_line((x + w, y + 1), (x + w, y + h / 2 - 1))?;
                    c.draw_line((x, y + h / 2), (x + w - 1, y + h / 2))?;
                }
                'O' => {
                    c.draw_line((x + 1, y), (x + w - 1, y))?;
                    c.draw_line((x + 1, y + h), (x + w - 1, y + h))?;
                    c.draw_line((x, y + 1), (x, y + h - 1))?;
                    c.draw_line((x + w, y + 1), (x + w, y + h - 1))?;
                }
                'I' => {
                    c.draw_line((x, y), (x + w, y))?;
                    c.draw_line((x + w / 2, y), (x + w / 2, y + h))?;
                    c.draw_line((x, y + h), (x + w, y + h))?;
                }
                'S' => {
                    c.draw_line((x + 1, y), (x + w, y))?;
                    c.draw_line((x, y + 1), (x, y + h / 2 - 1))?;
                    c.draw_line((x + w, y + h / 2 + 1), (x + w, y + h - 1))?;
                    c.draw_line((x + 1, y + h / 2), (x + w - 1, y + h / 2))?;
                    c.draw_line((x, y + h), (x + w - 1, y + h))?;
                }
                'L' => {
                    c.draw_line((x, y), (x, y + h))?;
                    c.draw_line((x, y + h), (x + w, y + h))?;
                }
                'V' => {
                    c.draw_line((x, y), (x + w / 2, y + h))?;
                    c.draw_line((x + w / 2, y + h), (x + w, y))?;
                }
                _ => {}
            }
            x += w + s;
        }
        Ok(())
    }
}
struct DrawingContext {
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
    fn new(vp_w: u32, vp_h: u32, board_w: u32, board_h: u32, board_b: u32) -> DrawingContext {
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
    fn draw<T: RenderTarget>(&self, c: &mut Canvas<T>, g: &Game) -> Result<(), String> {
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
struct Config {
    btype: u32,
    level: u32,
}
struct Game<'a> {
    config: Config,
    status: Status,
    board: Imprint,
    gravity_tick: u32,
    speed: u32,
    current: Piece,
    next: Piece,
    remaining: i32,
    position: (i32, i32),
    input: InputState,
    lines: Vec<usize>,
    points: u32,
    drop_rate: u32,
    score_table: ScoreTable<'a>,
}

impl<'a> Game<'a> {
    fn new(filename: &'a Path) -> std::io::Result<Self> {
        let mut g = Game {
            config: Config { btype: 0, level: 0 },
            status: Status::Menu(0),
            board: Imprint::empty(WIDTH, HEIGHT + BUFFER),
            gravity_tick: 0,
            speed: FRAMERATE,
            remaining: ADVANCE_SPEED,
            drop_rate: 0,
            current: Piece::I2,
            next: Piece::I2,
            position: (0, 0),
            points: 0,
            score_table: ScoreTable::new(filename)?,
            input: InputState {
                skip: 0,
                escape: false,
                down: false,
                left: false,
                right: false,
                rotate_l: false,
                rotate_r: false,
                drop: false,
            },
            lines: Vec::new(),
        };
        g.new_piece();
        g.new_piece();
        Ok(g)
    }
    fn new_game(&mut self) {
        self.board = Imprint::empty(WIDTH, HEIGHT + BUFFER);
        self.score_table
            .update_scores(&self.config, self.points)
            .unwrap();
        self.new_piece();
        self.new_piece();
        self.gravity_tick = 0;
        self.points = 0;
        self.drop_rate = 0;
        self.speed = FRAMERATE - self.config.level;
        self.remaining = (self.config.level + 1) as i32 * ADVANCE_SPEED;
        for i in 0..self.config.btype {
            let top = self.board.size().1 - 1 - i as usize;
            self.board.random_line(top);
        }
    }
    fn current_level(&self) -> u32 {
        FRAMERATE - self.speed
    }
    fn new_piece(&mut self) {
        self.current = self.next;
        self.next = rand::random::<Piece>();
        let x = (WIDTH as i32 - self.current.imprint().size().0 as i32) / 2;
        let y = if self.current == Piece::I1 { 0 } else { 1 };
        if !self.move_piece(x, y) || !self.board.all_clear(BUFFER) {
            self.status = Status::Raising(self.board.size().1);
        }
    }
    fn award_points(&mut self, lines: u32) {
        let level = (FRAMERATE - self.speed) + 1;
        let award = match lines {
            1 => 40 * level,
            2 => 100 * level,
            3 => 300 * level,
            _ => 1200 * level,
        };
        self.points += award
    }

    fn switch_piece(&mut self, p: Piece) -> bool {
        if self.board.accepts(p.imprint(), self.position) {
            self.current = p;
            true
        } else {
            false
        }
    }
    fn move_piece(&mut self, x: i32, y: i32) -> bool {
        let c = (x, y);
        if self.board.accepts(self.current.imprint(), c) {
            self.position = c;
            true
        } else {
            false
        }
    }
    fn hard_drop(&mut self) {
        while self.status == Status::Active {
            self.drop_rate += 1;
            self.down();
        }
    }
    fn rotate_l(&mut self) {
        let p = self.current.rotate_l();
        self.switch_piece(p);
    }
    fn rotate_r(&mut self) {
        let p = self.current.rotate_r();
        self.switch_piece(p);
    }
    fn check_lines(&mut self) -> bool {
        self.board.full_lines(&mut self.lines)
    }

    fn clear_lines(&mut self) {
        let lines = self.lines.len() as u32;
        self.award_points(lines);
        self.board.clear_lines(&mut self.lines)
    }
    fn down(&mut self) {
        let (x, y) = self.position;
        if !self.move_piece(x, y + 1) {
            self.points += self.drop_rate;
            self.drop_rate = 0;
            self.board.stamp(self.current.imprint(), self.position);
            if !self.check_lines() {
                self.status = Status::Placing(self.current, x, y);
            } else {
                self.remaining -= self.lines.len() as i32;
                while self.remaining <= 0 {
                    self.remaining += ADVANCE_SPEED;
                    if self.speed > 1 {
                        self.speed -= 1;
                    }
                }
                self.status = Status::Clearing(self.lines.len() as i32 * 3);
            }
        }
    }
    fn left(&mut self) {
        let (x, y) = self.position;
        self.move_piece(x - 1, y);
    }
    fn right(&mut self) {
        let (x, y) = self.position;
        self.move_piece(x + 1, y);
    }
    fn tick(&mut self) -> bool {
        match self.status {
            Status::Active => {
                if self.input.escape {
                    self.status = Status::Paused;
                    self.input.escape = false;
                } else {
                    if self.input.left {
                        if self.input.skip == 0 || self.input.skip > KEY_DELAY {
                            self.left();
                        }
                        if self.input.skip <= KEY_DELAY {
                            self.input.skip += 1;
                        }
                    } else if self.input.right {
                        if self.input.skip == 0 || self.input.skip > KEY_DELAY {
                            self.right();
                        }
                        if self.input.skip <= KEY_DELAY {
                            self.input.skip += 1;
                        }
                    }
                    if self.input.rotate_r {
                        self.rotate_r();
                        self.input.rotate_r = false;
                    } else if self.input.rotate_l {
                        self.rotate_l();
                        self.input.rotate_l = false;
                    }
                    if self.input.drop {
                        self.input.drop = false;
                        self.hard_drop()
                    } else if self.input.down {
                        self.drop_rate += 1;
                        self.down()
                    } else {
                        self.drop_rate = 0;
                        self.gravity_tick = (self.gravity_tick + 1) % self.speed;
                        if self.gravity_tick == 0 {
                            self.down()
                        }
                    }
                }
            }
            Status::Paused => {
                if self.input.drop || self.input.rotate_l || self.input.rotate_r ||
                    self.input.left || self.input.right || self.input.down
                {
                    self.status = Status::Active;
                } else if self.input.escape {
                    self.input.escape = false;
                    self.status = Status::Raising(self.board.size().1);
                }
            }

            Status::Raising(f) => {
                if f == BUFFER {
                    self.new_game();
                    self.status = Status::Lowering(f)
                } else {
                    self.status = Status::Raising(f - 1);
                }
            }
            Status::Lowering(f) => {
                if f == self.board.size().1 {
                    self.status = Status::Menu(0);
                } else {
                    self.status = Status::Lowering(f + 1);
                }
            }
            Status::Menu(f) => {
                self.status = Status::Menu((f + 1) % 70);
                if self.input.escape {
                    self.input.escape = false;
                    return false;
                }
                if self.input.right && self.config.btype < MAX_BTYPE {
                    self.input.right = false;
                    let top = self.board.size().1 - 1 - self.config.btype as usize;
                    self.board.random_line(top);
                    self.config.btype += 1;
                }
                if self.input.left && self.config.btype > 0 {
                    self.input.left = false;
                    self.config.btype -= 1;
                    let top = self.board.size().1 - 1 - self.config.btype as usize;
                    self.board.clear_line(top);
                }
                if self.input.drop {
                    self.input.drop = false;
                    self.status = Status::Active;
                }
                if self.input.rotate_l || self.input.rotate_r {
                    self.input.rotate_r = false;
                    self.input.rotate_l = false;
                    if self.config.level < FRAMERATE - 1 {
                        self.config.level += 1;
                        self.speed = FRAMERATE - self.config.level;
                        self.remaining = (self.config.level + 1) as i32 * ADVANCE_SPEED;
                    }
                }
                if self.input.down {
                    self.input.down = false;
                    if self.config.level > 0 {
                        self.config.level -= 1;
                        self.speed = FRAMERATE - self.config.level;
                        self.remaining = (self.config.level + 1) as i32 * ADVANCE_SPEED;
                    }
                }
            }
            Status::Clearing(0) => {
                self.clear_lines();
                self.status = Status::Active;
                self.new_piece();
            }
            Status::Clearing(f) => self.status = Status::Clearing(f - 1),
            Status::Placing(_, _, _) => {
                self.status = Status::Active;
                self.new_piece();
            }
        }
        true
    }
}


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
        WIDTH as u32,
        (HEIGHT + BUFFER) as u32,
        BUFFER as u32,
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
                WIDTH as u32,
                (HEIGHT + BUFFER) as u32,
                BUFFER as u32,
            );
        }
        ctx.draw(&mut canvas, &game).unwrap();
        // The rest of the game loop goes here...
        rate_limiter.delay();
    }
}
