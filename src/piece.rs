use cell::{Cell};
use imprint::{Imprint};
use rand::{Rand, Rng};
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    I1,
    I2,
    O1,
    J1,
    J2,
    J3,
    J4,
    L1,
    L2,
    L3,
    L4,
    S1,
    S2,
    T1,
    T2,
    T3,
    T4,
    Z1,
    Z2,
}
impl Rand for Piece {
    fn rand<R : Rng>(rng : &mut R) -> Piece {
        let x : u8 = rng.gen_range(0,7);
        match x {
           0 => Piece::I1,
           1 => Piece::O1,
           2 => Piece::J1,
           3 => Piece::L1,
           4 => Piece::S1,
           5 => Piece::T1,
           6 => Piece::Z1,
           _ => Piece::I2,
        }
    }
}
impl Piece {
    pub fn imprint(&self) -> &Imprint {
        lazy_static! {
            static ref II1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0, 0],
                  &[0, 0, 0, 0],
                  &[1, 1, 1, 1],
                  &[0, 0, 0, 0]],
                Cell::Filled,
            );
            static ref II2: Imprint = Imprint::from_footprint(
                &[&[0, 0, 1, 0],
                  &[0, 0, 1, 0],
                  &[0, 0, 1, 0],
                  &[0, 0, 1, 0]],
                Cell::Filled,
            );
            static ref IS1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0],
                  &[0, 1, 1],
                  &[1, 1, 0]],
                Cell::Filled,
            );
            static ref IS2: Imprint = Imprint::from_footprint(
                &[&[0, 1, 0],
                  &[0, 1, 1],
                  &[0, 0, 1]],
                Cell::Filled,
            );
            static ref IZ1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0],
                  &[1, 1, 0],
                  &[0, 1, 1]],
                Cell::Filled,
            );
            static ref IZ2: Imprint = Imprint::from_footprint(
                &[&[0, 0, 1],
                  &[0, 1, 1],
                  &[0, 1, 0]],
                Cell::Filled,
            );
            static ref IO1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0, 0],
                  &[0, 1, 1, 0],
                  &[0, 1, 1, 0],
                  &[0, 0, 0, 0]],
                Cell::Filled,
            );
            static ref IJ1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0],
                  &[1, 1, 1],
                  &[0, 0, 1]],
                Cell::Filled,
            );
            static ref IJ2: Imprint = Imprint::from_footprint(
                &[&[0, 1, 0],
                  &[0, 1, 0],
                  &[1, 1, 0]],
                Cell::Filled,
            );
            static ref IJ3: Imprint = Imprint::from_footprint(
                &[&[1, 0, 0],
                  &[1, 1, 1],
                  &[0, 0, 0]],
                Cell::Filled,
            );
            static ref IJ4: Imprint = Imprint::from_footprint(
                &[&[0, 1, 1],
                  &[0, 1, 0],
                  &[0, 1, 0]],
                Cell::Filled,
            );
            static ref IL1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0],
                  &[1, 1, 1],
                  &[1, 0, 0]],
                Cell::Filled,
            );
            static ref IL2: Imprint = Imprint::from_footprint(
                &[&[1, 1, 0],
                  &[0, 1, 0],
                  &[0, 1, 0]],
                Cell::Filled,
            );
            static ref IL3: Imprint = Imprint::from_footprint(
                &[&[0, 0, 1],
                  &[1, 1, 1],
                  &[0, 0, 0]],
                Cell::Filled,
            );
            static ref IL4: Imprint = Imprint::from_footprint(
                &[&[0, 1, 0],
                  &[0, 1, 0],
                  &[0, 1, 1]],
                Cell::Filled,
            );
            static ref IT1: Imprint = Imprint::from_footprint(
                &[&[0, 0, 0],
                  &[1, 1, 1],
                  &[0, 1, 0]],
                Cell::Filled,
            );
            static ref IT2: Imprint = Imprint::from_footprint(
                &[&[0, 1, 0],
                  &[1, 1, 0],
                  &[0, 1, 0]],
                Cell::Filled,
            );
            static ref IT3: Imprint = Imprint::from_footprint(
                &[&[0, 1, 0],
                  &[1, 1, 1],
                  &[0, 0, 0]],
                Cell::Filled,
            );
            static ref IT4: Imprint = Imprint::from_footprint(
                &[&[0, 1, 0],
                  &[0, 1, 1],
                  &[0, 1, 0]],
                Cell::Filled,
            );
        }
        match *self {
            Piece::I1 => &II1,
            Piece::I2 => &II2,
            Piece::S1 => &IS1,
            Piece::S2 => &IS2,
            Piece::Z1 => &IZ1,
            Piece::Z2 => &IZ2,
            Piece::O1 => &IO1,
            Piece::J1 => &IJ1,
            Piece::J2 => &IJ2,
            Piece::J3 => &IJ3,
            Piece::J4 => &IJ4,
            Piece::L1 => &IL1,
            Piece::L2 => &IL2,
            Piece::L3 => &IL3,
            Piece::L4 => &IL4,
            Piece::T1 => &IT1,
            Piece::T2 => &IT2,
            Piece::T3 => &IT3,
            Piece::T4 => &IT4,
        }
    }
    pub fn rotate_r(&self) -> Piece {
        match *self {
            Piece::I1 => Piece::I2,
            Piece::I2 => Piece::I1,
            Piece::S1 => Piece::S2,
            Piece::S2 => Piece::S1,
            Piece::Z1 => Piece::Z2,
            Piece::Z2 => Piece::Z1,
            Piece::O1 => Piece::O1,
            Piece::J1 => Piece::J2,
            Piece::J2 => Piece::J3,
            Piece::J3 => Piece::J4,
            Piece::J4 => Piece::J1,
            Piece::L1 => Piece::L2,
            Piece::L2 => Piece::L3,
            Piece::L3 => Piece::L4,
            Piece::L4 => Piece::L1,
            Piece::T1 => Piece::T2,
            Piece::T2 => Piece::T3,
            Piece::T3 => Piece::T4,
            Piece::T4 => Piece::T1,
        }
    }
    pub fn rotate_l(&self) -> Piece {
        self.rotate_r().rotate_r().rotate_r()
    }
}
