use sdl2::render::RenderTarget;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

use imprint::Imprint;

pub struct BoardDrawingContext {
    pub offset_x: u32,
    pub offset_y: u32,
    pub box_w: u32,
    pub box_h: u32,
    pub buffer_h: u32,
    pub board_w: u32,
    pub board_h: u32,
}

impl BoardDrawingContext {
    pub fn draw_box<T: RenderTarget>(&self, c: &mut Canvas<T>, px: i32, py: i32) -> Result<(), String> {
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
    pub fn fill_rect<T: RenderTarget>(
        &self,
        c: &mut Canvas<T>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Result<(), String> {
        for y in y1..y2 {
            for x in x1..x2 {
                self.draw_box(c,x,y)?;
            }
        }
        Ok(())
    }
    pub fn fill_boxes<T: RenderTarget>(
        &self,
        c: &mut Canvas<T>,
        y1: i32,
        y2: i32,
    ) -> Result<(), String> {
        self.fill_rect(c,0,y1,self.board_w as i32, y2)
    }

    pub fn fill_all_boxes<T: RenderTarget>(&self, c: &mut Canvas<T>) -> Result<(), String> {
        self.fill_boxes(c, self.buffer_h as i32, self.board_h as i32)
    }

    pub fn draw_imprint<A : Copy,T: RenderTarget>(
        &self,
        c: &mut Canvas<T>,
        p: &Imprint<A>,
        x: i32,
        y: i32,
    ) -> Result<(), String> {
        let (w, h) = p.size();
        for cy in 0..h {
            for cx in 0..w {
                if !p[(cx, cy)].is_empty() {
                    self.draw_box(c, x + cx as i32, y + cy as i32)?;
                }
            }
        }
        Ok(())
    }
}
