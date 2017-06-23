use std::ops::{Index, IndexMut};
use rand;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Filled,
}

pub struct Imprint {
    footprint: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Imprint {

    pub fn empty(width: usize, height: usize) -> Imprint {
        Imprint {
            footprint: vec![Cell::Empty; width * height],
            width: width,
            height: height,
        }
    }

    pub fn from_footprint(print: &[&[u8]], style: Cell) -> Imprint {
        let h = print.len();
        let w = print[0].len();
        let mut it = Imprint::empty(w, h);
        for y in 0..h {
            for x in 0..w {
                it[(x, y)] = if print[y][x] > 0 { style } else { Cell::Empty };
            }
        }
        it
    }

    pub fn full_lines(&self, results: &mut Vec<usize>) -> bool {
        let mut all = false;
        for y in 0..self.height {
            let mut row = true;
            for x in 0..self.width {
                row = row && self[(x, y)] != Cell::Empty;
            }
            if row {
                results.push(y)
            }
            all = all || row;
        }
        all
    }

    fn copy_row(&mut self, src: i32, dest: usize) {
        if src == dest as i32 {
            return;
        };
        if src < 0 {
            for i in 0..self.width {
                self[(i, dest)] = Cell::Empty;
            }
        } else {
            for i in 0..self.width {
                self[(i, dest)] = self[(i, src as usize)];
            }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    //clear line without moving others down
    pub fn clear_line(&mut self, line: usize) {
        for x in 0..self.width {
            self[(x,line)] = Cell::Empty;
        }
    }

    //randomise a line
    pub fn random_line(&mut self, line: usize) {
        for x in 0..self.width {
            self[(x,line)] =  Cell::Filled;
        }
        for _ in 0..(self.width / 2) {
            let x = rand::random::<usize>() % self.width;
            self[(x, line)] = Cell::Empty;
        }
    }

    //precondition: lines is a sorted vector of line indices.
    pub fn clear_lines(&mut self, lines: &mut Vec<usize>) {
        let mut n = lines.pop().unwrap_or(self.height);
        let mut sy = self.height as i32 - 1;
        for y in (0..self.height).rev() {
            while sy == n as i32 {
                sy -= 1;
                n = lines.pop().unwrap_or(self.height);
            }
            self.copy_row(sy,y);
            sy -= 1;
        }
    }

    pub fn accepts(&self, other: &Imprint, (x0, y0): (i32, i32)) -> bool {
        for y in 0..other.height {
            for x in 0..other.width {
                if other[(x, y)] != Cell::Empty {
                    let xx = x0 + x as i32;
                    let yy = y0 + y as i32;
                    if xx >= self.width as i32 || yy >= self.height as i32 || xx < 0 || yy < 0 {
                        return false;
                    }
                    if self[(xx as usize, yy as usize)] != Cell::Empty {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn all_clear(&self, range: usize) -> bool {
        for y in 0..range {
            for x in 0..self.width {
                if self[(x, y)] != Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn stamp(&mut self, other: &Imprint, (x0, y0): (i32, i32)) {
        for y in 0..other.height {
            for x in 0..other.width {
                if other[(x, y)] != Cell::Empty {
                    let xx = x0 + x as i32;
                    let yy = y0 + y as i32;
                    if xx < self.width as i32 && yy < self.height as i32 && xx >= 0 && yy >= 0 {
                        self[(xx as usize, yy as usize)] = other[(x, y)];
                    }
                }
            }
        }
    }
}

impl Index<(usize, usize)> for Imprint {
    type Output = Cell;
    fn index<'a>(&'a self, (x, y): (usize, usize)) -> &'a Cell {
        &self.footprint[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Imprint {
    fn index_mut<'a>(&'a mut self, (x, y): (usize, usize)) -> &'a mut Cell {
        &mut self.footprint[y * self.width + x]
    }
}
