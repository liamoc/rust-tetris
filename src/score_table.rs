use std::path::Path;
use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use game;


pub struct ScoreTable<'a> {
    scores: [[u32; game::MAX_BTYPE as usize + 1]; game::MAX_LEVEL as usize],
    filename: &'a Path,
}

impl<'a> ScoreTable<'a> {
    pub fn new(filename: &Path) -> ::std::io::Result<ScoreTable> {
        let mut it = ScoreTable {
            scores: [[0; game::MAX_BTYPE as usize + 1]; game::MAX_LEVEL as usize],
            filename: filename,
        };
        match File::open(it.filename) {
            Ok(mut file) => {
                for i in 0..game::MAX_LEVEL as usize {
                    for j in 0..game::MAX_BTYPE as usize + 1 {
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
        for i in 0..game::MAX_LEVEL as usize {
            for j in 0..game::MAX_BTYPE as usize + 1 {
                file.write_u32::<LittleEndian>(self.scores[i][j])?;
            }
        }
        Ok(())
    }
    pub fn get_top_score(&self, c: &game::Config) -> u32 {
        self.scores[c.level as usize][c.btype as usize]
    }

    pub fn update_scores(&mut self, c: &game::Config, score: u32) -> ::std::io::Result<()> {
        if self.scores[c.level as usize][c.btype as usize] < score {
            self.scores[c.level as usize][c.btype as usize] = score;
            self.save_scores()?;
        }
        Ok(())
    }
}
