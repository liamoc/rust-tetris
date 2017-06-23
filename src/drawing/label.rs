use sdl2::render::RenderTarget;
use sdl2::render::Canvas;

pub struct LabelDrawingContext {
    pub w: i32,
    pub h: i32,
    pub spacing: i32,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl LabelDrawingContext {
    pub fn draw<T: RenderTarget>(
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
