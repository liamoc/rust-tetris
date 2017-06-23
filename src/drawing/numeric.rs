use sdl2::render::RenderTarget;
use sdl2::render::Canvas;


pub struct NumericDrawingContext {
    pub max_digits: u32,
    pub x: i32,
    pub y: i32,
    pub cell_w: i32,
    pub h: i32,
    pub spacing: i32,
}

impl NumericDrawingContext {
    pub fn draw_bg<T: RenderTarget>(&self, c: &mut Canvas<T>) -> Result<(), String> {
        self.draw(c, DigitsBG::new(self.max_digits, 8))
    }

    pub fn draw_num<T: RenderTarget>(&self, c: &mut Canvas<T>, num: u32) -> Result<(), String> {
        let max = 10u32.pow(self.max_digits) - 1;
        self.draw(c, Digits::new(if num <= max { num } else { max }))
    }

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

    fn draw<I, T: RenderTarget>(&self, c: &mut Canvas<T>, num: I) -> Result<(), String>
    where
        I: Iterator<Item = u32>,
    {
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
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                8 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    Self::draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                7 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                6 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    Self::draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                5 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                    Self::draw_vert_segment(c, x, y, y + h / 2, fat)?;
                }
                4 => {
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                3 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                2 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h / 2, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    Self::draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                }
                1 => {
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;

                }
                0 => {
                    Self::draw_horiz_segment(c, x, x + w, y, fat)?;
                    Self::draw_horiz_segment(c, x, x + w, y + h, fat)?;
                    Self::draw_vert_segment(c, x, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x, y + h / 2, y + h, fat)?;
                    Self::draw_vert_segment(c, x + w, y, y + h / 2, fat)?;
                    Self::draw_vert_segment(c, x + w, y + h / 2, y + h, fat)?;
                }
                _ => {}
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
