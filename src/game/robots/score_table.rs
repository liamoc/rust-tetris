use std::path::Path;
use std::fs::File;

use super::MAX_ROBOTS;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};


pub struct ScoreTable<'a> {
    scores: [[u32; MAX_ROBOTS as usize + 1]; super::MAX_LEVEL as usize],
    filename: &'a Path,
}

impl<'a> ScoreTable<'a> {
    pub fn new(filename: &Path) -> ::std::io::Result<ScoreTable> {
        let mut it = ScoreTable {
            scores: [[0; MAX_ROBOTS as usize + 1]; super::MAX_LEVEL as usize],
            filename: filename,
        };
        match File::open(it.filename) {
            Ok(mut file) => {
                for i in 0..super::MAX_LEVEL as usize {
                    for j in 0..MAX_ROBOTS as usize + 1 {
                        it.scores[i][j] = file.read_u32::<LittleEndian>()?;
                    }
                }
            }
            Err(_) => {}
        }
        Ok(it)
    }

    pub fn save_scores(&self) -> ::std::io::Result<()> {
        let mut file = File::create(self.filename)?;
        for i in 0..super::MAX_LEVEL as usize {
            for j in 0..MAX_ROBOTS as usize + 1 {
                file.write_u32::<LittleEndian>(self.scores[i][j])?;
            }
        }
        Ok(())
    }
    pub fn get_top_score(&self, c: &super::Config) -> u32 {
        self.scores[c.level as usize][c.robots as usize]
    }

    pub fn update_scores(&mut self, c: &super::Config, score: u32) -> ::std::io::Result<()> {
        if self.scores[c.level as usize][c.robots as usize] < score {
            self.scores[c.level as usize][c.robots as usize] = score;
            self.save_scores()?;
        }
        Ok(())
    }
}
